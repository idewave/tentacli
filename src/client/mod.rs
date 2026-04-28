use async_broadcast::broadcast;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::Context;
use cfg_if::cfg_if;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde::de::DeserializeOwned;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub mod plugin;
pub mod packet;
pub mod types;
mod transport;
pub mod prelude;

pub use prelude::*;

inventory::collect!(PluginLoader<dyn NetworkPlugin>);
inventory::collect!(PluginLoader<dyn CorePlugin>);
inventory::collect!(PluginLoader<dyn ProcessorPlugin>);

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty, $trait_obj:ty) => {
        inventory::submit! {
            PluginLoader::<$trait_obj> {
                load: || Arc::new(<$plugin_type>::default()) as Arc<$trait_obj>,
                name: stringify!($plugin_type),
            }
        }
    };
}

cfg_if! {
    if #[cfg(feature = "wow-wotlk")] {
        #[cfg(not(feature = "replay"))]
        use crate::plugins::wow::wotlk::login;
        #[cfg(not(feature = "replay"))]
        use crate::plugins::wow::logger::Logger;
        use crate::plugins::wow::wotlk::realm;

        // network plugins
        #[cfg(not(feature = "replay"))]
        register_plugin!(login::LoginPlugin, dyn NetworkPlugin);
        #[cfg(feature = "replay")]
        register_plugin!(OutgoingPolicy<realm::RealmPlugin, false>, dyn NetworkPlugin);

        #[cfg(not(feature = "replay"))]
        register_plugin!(realm::RealmPlugin, dyn NetworkPlugin);

        // processor plugins
        #[cfg(not(feature = "replay"))]
        register_plugin!(login::Processors, dyn ProcessorPlugin);
        register_plugin!(realm::Processors, dyn ProcessorPlugin);
        #[cfg(not(feature = "replay"))]
        register_plugin!(Logger, dyn CorePlugin);
    }
}

cfg_if! {
    if #[cfg(feature = "replay")] {
        use crate::plugins::replay::Replay;
        register_plugin!(Replay, dyn CorePlugin);
    }
}

// core plugins
cfg_if! {
    if #[cfg(feature = "tui")] {
        use crate::plugins::tui::TUIPlugin;

        register_plugin!(TUIPlugin, dyn CorePlugin);
    } else if #[cfg(feature = "dbg-ui")] {
        use crate::plugins::dbg_ui::DbgUI;

        register_plugin!(DbgUI, dyn CorePlugin);
    }
}

register_plugin!(crate::plugins::core::Core, dyn CorePlugin);

pub struct Client;
impl Client {
    fn collect_labels() -> (Vec<ServerLabel>, Vec<ServerLabel>) {
        let network_labels: Vec<ServerLabel> =
            inventory::iter::<PluginLoader<dyn NetworkPlugin>>
                .into_iter()
                .map(|loader| (loader.load)().label())
                .collect();

        let processor_labels: Vec<ServerLabel> =
            inventory::iter::<PluginLoader<dyn ProcessorPlugin>>
                .into_iter()
                .map(|loader| (loader.load)().label())
                .collect();

        (network_labels, processor_labels)
    }

    fn validate_labels(
        network_labels: &[ServerLabel],
        processor_labels: &[ServerLabel],
    ) -> Result<(), ValidationError> {
        let mut network_set = HashSet::with_capacity(network_labels.len());

        // Check duplicates and build set
        for &label in network_labels {
            if !network_set.insert(label) {
                return Err(ValidationError::DuplicateNetworkLabel(label));
            }
        }

        // Check processors are wired to a network
        for &label in processor_labels {
            if !network_set.contains(&label) {
                return Err(ValidationError::MissingNetworkForProcessor(label));
            }
        }

        Ok(())
    }

    pub async fn run(context: Option<SharedContext>) -> anyhow::Result<()> {
        let context = context.unwrap_or_else(|| {
            Arc::new(RwLock::new(CtxMap::default()))
        });

        // Root cancellation token for the whole system
        let shutdown = CancellationToken::new();

        let mut echo_senders: HashMap<ServerLabel, Sender<Echo>> = HashMap::new();
        let (broadcast_tx, mut broadcast_rx) = broadcast::<OrderedOutput>(100);
        broadcast_rx.set_overflow(true);

        let mut tasks: Vec<Task> = vec![];

        // Collect and validate plugin labels
        let (network_labels, processor_labels) = Client::collect_labels();

        Client::validate_labels(&network_labels, &processor_labels)
            .map_err(|err| match err {
                ValidationError::DuplicateNetworkLabel(label) => {
                    anyhow::anyhow!("Duplicate NetworkPlugin label \"{}\"", label)
                }
                ValidationError::MissingNetworkForProcessor(label) => {
                    anyhow::anyhow!("No NetworkPlugin was registered with label \"{}\"", label)
                }
            })?;

        // Start network plugins
        for loader in inventory::iter::<PluginLoader<dyn NetworkPlugin>> {
            let plugin: Arc<dyn NetworkPlugin> = (loader.load)();

            let (echo_tx, echo_rx) = mpsc::channel::<Echo>(100);
            let (packet_sender, packet_receiver) = mpsc::channel::<Vec<Packet>>(100);

            echo_senders.insert(plugin.label(), echo_tx.clone());

            let task = plugin.connect(
                echo_rx,
                echo_tx,
                packet_sender,
                packet_receiver,
                broadcast_tx.clone(),
                shutdown.clone(),
                context.clone(),
            );

            tasks.push(task);
        }

        let echo_senders = Arc::new(echo_senders);

        // Start core plugins
        for loader in inventory::iter::<PluginLoader<dyn CorePlugin>> {
            let plugin = (loader.load)();
            tasks.extend(
                plugin.get_tasks(
                    broadcast_rx.clone(),
                    echo_senders.clone(),
                    shutdown.clone(),
                    context.clone(),
                )?
            );
        }

        // Drive all tasks and broadcast errors
        let mut futures: FuturesUnordered<_> = tasks.into_iter().collect();

        while let Some(result) = futures.next().await {
            let err_message = match result {
                Ok(Err(err)) => Some(err.to_string()),
                Err(err) => Some(err.to_string()),
                _ => None,
            };

            if let Some(message) = err_message {
                let payload = Arc::new(vec![
                    HandlerOutput::Messages(vec![
                        Message {
                            msg_type: MsgType::Error,
                            text: message,
                        }
                    ]),
                ]);

                for label in network_labels.iter().copied() {
                    broadcast_tx
                        .broadcast(OrderedOutput::new(label, payload.clone()))
                        .await?;
                }
            }
        }

        Ok(())
    }

    fn snapshot_plugins() -> PluginSnapshot {
        let networks = inventory::iter::<PluginLoader<dyn NetworkPlugin>>
            .into_iter()
            .map(|l| {
                let plugin = (l.load)();
                (plugin.label(), l.name)
            })
            .collect::<Vec<_>>();

        let processors = inventory::iter::<PluginLoader<dyn ProcessorPlugin>>
            .into_iter()
            .map(|l| {
                let plugin = (l.load)();
                (plugin.label(), l.name)
            })
            .collect::<Vec<_>>();

        let cores = inventory::iter::<PluginLoader<dyn CorePlugin>>
            .into_iter()
            .map(|l| l.name)
            .collect::<Vec<_>>();

        PluginSnapshot {
            networks,
            processors,
            cores,
        }
    }

    pub fn doctor() -> anyhow::Result<()> {
        println!("Tentacli Doctor\n");

        println!("[OK] Build features");
        println!("  - tui        : {}", cfg!(feature = "tui"));
        println!("  - dbg-ui     : {}", cfg!(feature = "dbg-ui"));
        println!("  - wow-wotlk  : {}", cfg!(feature = "wow-wotlk"));
        println!("  - replay     : {}", cfg!(feature = "replay"));

        let snapshot = Client::snapshot_plugins();

        println!("\n[OK] Plugins");

        println!("  - Network:");
        if snapshot.networks.is_empty() {
            println!("      <none>");
        } else {
            for (label, name) in &snapshot.networks {
                println!("      {}  ({})", label, name);
            }
        }

        println!("  - Processor:");
        if snapshot.processors.is_empty() {
            println!("      <none>");
        } else {
            for (label, name) in &snapshot.processors {
                println!("      {}  ({})", label, name);
            }
        }

        println!("  - Core:");
        if snapshot.cores.is_empty() {
            println!("      <none>");
        } else {
            for name in &snapshot.cores {
                println!("      {}", name);
            }
        }

        let network_labels: Vec<ServerLabel> =
            snapshot.networks.iter().map(|(l, _)| *l).collect();

        let processor_labels: Vec<ServerLabel> =
            snapshot.processors.iter().map(|(l, _)| *l).collect();

        Client::validate_labels(&network_labels, &processor_labels)
            .map_err(|e| anyhow::anyhow!("Wiring error: {:?}", e))?;

        println!("\n[OK] Wiring");
        println!("  - All processor plugins are bound to a network plugin");

        println!("\n[INFO] Config lookup order");

        if let Some(dir) = env::var_os("TENTACLI_CONFIG_DIR") {
            println!("  - TENTACLI_CONFIG_DIR = {:?}", dir);
        } else {
            println!("  - TENTACLI_CONFIG_DIR = <not set>");
        }

        if let Some(dir) = dirs_next::config_dir() {
            println!("  - OS config dir       = {:?}", dir);
        } else {
            println!("  - OS config dir       = <not available>");
        }

        if let Ok(cwd) = env::current_dir() {
            println!("  - Current dir        = {:?}", cwd);
        }

        println!("\n[OK] Doctor finished");
        Ok(())
    }
}

struct PluginSnapshot {
    networks: Vec<(ServerLabel, &'static str)>,
    processors: Vec<(ServerLabel, &'static str)>,
    cores: Vec<&'static str>,
}

#[derive(Debug, PartialEq, Eq)]
enum ValidationError {
    DuplicateNetworkLabel(ServerLabel),
    MissingNetworkForProcessor(ServerLabel),
}

pub struct ConfigParser;
impl ConfigParser {
    pub fn parse_from_file<T, P>(path: P) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = Self::get_cfg_path(path.as_ref())?;
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config at {}", path.display()))?;

        Self::parse_from_string(text)
    }

    pub fn parse_from_string<T: DeserializeOwned>(cfg_string: String) -> anyhow::Result<T> {
        toml::from_str(&cfg_string).with_context(|| "Failed to parse TOML")
    }

    fn get_cfg_path(relative_path: &Path) -> anyhow::Result<PathBuf> {
        // Looking for configs with TENTACLI_CONFIG_DIR as the config dir
        if let Some(root_dir) = env::var_os("TENTACLI_CONFIG_DIR") {
            let candidate = PathBuf::from(root_dir).join(relative_path);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        // Check the default per-user configuration directory provided by the OS:
        //
        // - Linux / BSD:   $XDG_CONFIG_HOME or $HOME/.config
        // - Windows:       %APPDATA% (e.g. C:\Users\<User>\AppData\Roaming)
        // - macOS:         $HOME/Library/Application Support
        //
        // Append the Cargo package name to keep configs isolated per application.
        if let Some(root_dir) = dirs_next::config_dir() {
            let candidate = root_dir.join(env!("CARGO_PKG_NAME")).join(relative_path);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        // Current working directory: ./<path>
        // This makes it convenient to run the binary in a project folder
        // and have configs resolved relative to it.
        if let Ok(cwd) = env::current_dir() {
            let candidate = cwd.join(relative_path);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        // Development fallback: <CARGO_MANIFEST_DIR>/plugins/<path>
        // This covers `cargo run`, when configs are stored in the project root
        // under "plugins/". It uses the compile-time env var injected by Cargo.
        let candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/plugins")
            .join(relative_path);

        if candidate.is_file() {
            return Ok(candidate);
        }

        anyhow::bail!("Config file not found for {:?}", relative_path);
    }
}

pub struct PluginLoader<T: ?Sized + 'static> {
    pub load: fn() -> Arc<T>,
    pub name: &'static str,
}

pub type SharedContext = Arc<RwLock<CtxMap>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    struct EnvGuard {
        key: &'static str,
        old: Option<std::ffi::OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &Path) -> Self {
            let old = env::var_os(key);
            unsafe {
                env::set_var(key, value);
            }
            Self { key, old }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                if let Some(ref v) = self.old {
                    env::set_var(self.key, v);
                } else {
                    env::remove_var(self.key);
                }
            }
        }
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestConfig {
        server: ServerConfig,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct ServerConfig {
        host: String,
        port: u16,
    }

    #[test]
    fn parses_from_env_config_dir_with_sectioned_toml() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");

        let config_path = temp_dir.path().join("test.toml");

        let toml = r#"
[server]
host = "localhost"
port = 8080
"#;

        fs::write(&config_path, toml).expect("failed to write test config");

        let _guard = EnvGuard::set("TENTACLI_CONFIG_DIR", temp_dir.path());

        let result: TestConfig =
            ConfigParser::parse_from_file("test.toml").expect("parse failed");

        assert_eq!(
            result,
            TestConfig {
                server: ServerConfig {
                    host: "localhost".into(),
                    port: 8080
                }
            }
        );
    }

    #[test]
    fn valid_configuration_passes() {
        let networks: Vec<ServerLabel> = vec!["login", "realm"];
        let processors: Vec<ServerLabel> = vec!["login", "realm"];

        let result = Client::validate_labels(&networks, &processors);

        assert!(result.is_ok());
    }

    #[test]
    fn duplicate_network_label_is_detected() {
        let networks: Vec<ServerLabel> = vec!["login", "login"];
        let processors: Vec<ServerLabel> = vec!["login"];

        let result = Client::validate_labels(&networks, &processors);

        assert_eq!(
            result,
            Err(ValidationError::DuplicateNetworkLabel("login"))
        );
    }

    #[test]
    fn missing_network_for_processor_is_detected() {
        let networks: Vec<ServerLabel> = vec!["login"];
        let processors: Vec<ServerLabel> = vec!["realm"];

        let result = Client::validate_labels(&networks, &processors);

        assert_eq!(
            result,
            Err(ValidationError::MissingNetworkForProcessor("realm"))
        );
    }

    #[test]
    fn allows_no_processors() {
        let networks: Vec<ServerLabel> = vec!["login"];
        let processors: Vec<ServerLabel> = Vec::new();

        let result = Client::validate_labels(&networks, &processors);

        assert!(result.is_ok());
    }

    #[test]
    fn empty_networks_rejects_any_processor() {
        let networks: Vec<ServerLabel> = Vec::new();
        let processors: Vec<ServerLabel> = vec!["login"];

        let result = Client::validate_labels(&networks, &processors);

        assert_eq!(
            result,
            Err(ValidationError::MissingNetworkForProcessor("login"))
        );
    }
}
