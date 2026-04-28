#![cfg(feature = "wow-wotlk")]
//! Logger core plugin for WoW WotLK packet stream.
//!
//! Writes logs on-the-fly in a format compatible with MaNGOS-like engines
//! and supports size-based log fragmentation.

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_broadcast::{Receiver, RecvError};
use chrono::Local;
use serde::Deserialize;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm;
use crate::plugins::wow::wotlk::config::Config as WotlkConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub logs_dir: String,
    #[serde(default)]
    pub enabled: u8,
    #[serde(default)]
    pub max_file_size: u64,
}

#[derive(Default)]
pub struct Logger;

impl Logger {
    fn sanitize_segment(value: &str) -> String {
        let segment: String = value
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                    ch
                } else {
                    '_'
                }
            })
            .collect();

        if segment.is_empty() {
            "unknown".to_string()
        } else {
            segment
        }
    }

    fn resolve_logs_dir(configured: &str) -> PathBuf {
        let path = PathBuf::from(configured);

        if path.is_absolute() {
            return path;
        }

        if let Some(root_dir) = std::env::var_os("TENTACLI_CONFIG_DIR") {
            return PathBuf::from(root_dir).join(&path);
        }

        if let Some(root_dir) = dirs_next::config_dir() {
            return root_dir.join(env!("CARGO_PKG_NAME")).join(&path);
        }

        if let Ok(cwd) = std::env::current_dir() {
            return cwd.join(&path);
        }

        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/plugins")
            .join(&path)
    }

    fn resolve_log_file_suffix() -> Vec<String> {
        let mut parts = vec![];
        if let Ok(config) =
            ConfigParser::parse_from_file::<WotlkConfig, _>("wow/wotlk/connection.toml")
        {
            if let Some(connection) = config.connection
                && !connection.account_name.is_empty()
            {
                parts.push(Self::sanitize_segment(&connection.account_name));
            }

            if let Some(autoselect) = config.autoselect
                && !autoselect.realm_name.is_empty()
                && autoselect.realm_name != ".*"
            {
                parts.push(Self::sanitize_segment(&autoselect.realm_name));
            }
        }

        parts
    }

    fn build_log_file_path(logs_dir: &Path, suffix_parts: &[String]) -> PathBuf {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H-%M-%S").to_string();
        let mut parts = Vec::with_capacity(2 + suffix_parts.len());
        parts.push(date);
        parts.push(time);
        parts.extend(suffix_parts.iter().cloned());

        logs_dir.join(format!("{}.log", parts.join("_")))
    }

    fn build_fragment_file_path(base_path: &Path, fragment: u32) -> PathBuf {
        if fragment == 0 {
            return base_path.to_path_buf();
        }

        let stem = base_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("world");

        let extension = base_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("log");

        let name = format!("{stem}_{fragment}.{extension}");
        match base_path.parent() {
            Some(parent) => parent.join(name),
            None => PathBuf::from(name),
        }
    }

    fn format_entry(packet: &Packet, socket_port: u16) -> Option<Vec<u8>> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let opcode = match &packet.metadata.opcode {
            PacketOpcode::U16(value) => u32::from(*value),
            PacketOpcode::U32(value) => *value,
            _ => return None,
        };

        let opcode_name = if packet.metadata.packet_name.is_empty() {
            "UNKNOWN"
        } else {
            &packet.metadata.packet_name
        };

        let mut entry = Vec::with_capacity(96 + packet.content.body.len() * 4);
        write!(
            &mut entry,
            "{timestamp} \nSERVER:\nSOCKET: 127.0.0.1:{socket_port}\nLENGTH: {}\nOPCODE: {opcode_name} (0x{opcode:04X})\nDATA:\n",
            packet.content.body.len()
        )
        .ok()?;

        for chunk in packet.content.body.chunks(16) {
            for (i, byte) in chunk.iter().enumerate() {
                if i > 0 {
                    entry.push(b' ');
                }
                write!(&mut entry, "{byte:02X}").ok()?;
            }
            entry.push(b'\n');
        }

        entry.extend_from_slice(b"\n\n");
        Some(entry)
    }
}

impl CorePlugin for Logger {
    fn get_tasks(
        &self,
        broadcast_rx: Receiver<OrderedOutput>,
        _: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        let config: Config = ConfigParser::parse_from_file("wow/logger/logger.toml")?;
        if config.enabled == 0 {
            return Ok(vec![]);
        }

        let logs_dir = Self::resolve_logs_dir(&config.logs_dir);
        let log_suffix = Self::resolve_log_file_suffix();
        let log_file = Self::build_log_file_path(&logs_dir, &log_suffix);
        let max_file_size_bytes = config.max_file_size.saturating_mul(1024 * 1024);

        Ok(vec![tokio::spawn(async move {
            const QUEUE_SIZE: usize = 4096;
            let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(QUEUE_SIZE);

            let writer_task = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                fs::create_dir_all(&logs_dir)?;

                let mut fragment_idx: u32 = 0;
                let mut active_path = Self::build_fragment_file_path(&log_file, fragment_idx);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&active_path)?;
                let mut current_size = file.metadata()?.len();
                let mut writer = BufWriter::new(file);

                while let Some(entry) = write_rx.blocking_recv() {
                    if max_file_size_bytes > 0 && current_size + entry.len() as u64 > max_file_size_bytes {
                        writer.flush()?;

                        fragment_idx = fragment_idx.saturating_add(1);
                        active_path = Self::build_fragment_file_path(&log_file, fragment_idx);
                        file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&active_path)?;
                        current_size = file.metadata()?.len();
                        writer = BufWriter::new(file);
                    }

                    writer.write_all(&entry)?;
                    current_size = current_size.saturating_add(entry.len() as u64);
                }

                writer.flush()?;
                Ok(())
            });

            let mut ordered_rx = OrderedReceiver::new(broadcast_rx);

            loop {
                tokio::select! {
                    biased;
                    _ = shutdown.cancelled() => {
                        break;
                    },
                    result = ordered_rx.recv() => {
                        match result {
                            Ok(ordered) => {
                                if ordered.label != realm::PLUGIN_LABEL {
                                    continue;
                                }

                                for output in ordered.outputs.iter() {
                                    if let HandlerOutput::Packets(packets) = output {
                                        for packet in packets {
                                            if packet.metadata.packet_type != PacketType::Incoming {
                                                continue;
                                            }

                                            if let Some(entry) = Self::format_entry(packet, 0) {
                                                write_tx
                                                    .send(entry)
                                                    .await
                                                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                                            }
                                        }
                                    }
                                }
                            },
                            Err(RecvError::Overflowed(_)) => {
                                continue;
                            }
                            Err(RecvError::Closed) => {
                                break;
                            }
                        }
                    }
                }
            }

            drop(write_tx);

            match writer_task.await {
                Ok(result) => result?,
                Err(err) => return Err(err.into()),
            }

            Ok(())
        })])
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    use crate::client::ConfigParser;
    use crate::client::packet::{Packet, PacketOpcode, PacketType};
    use super::{Config, Logger};

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
                if let Some(value) = &self.old {
                    env::set_var(self.key, value);
                } else {
                    env::remove_var(self.key);
                }
            }
        }
    }

    #[test]
    fn sanitize_segment_replaces_unsupported_chars() {
        assert_eq!(Logger::sanitize_segment("A b/c:*?"), "A_b_c___");
        assert_eq!(Logger::sanitize_segment(""), "unknown");
    }

    #[test]
    fn resolve_logs_dir_uses_env_config_dir() {
        let temp_dir = tempfile::tempdir().expect("tempdir");
        let _guard = EnvGuard::set("TENTACLI_CONFIG_DIR", temp_dir.path());

        let resolved = Logger::resolve_logs_dir("wow/logger/logs");

        assert_eq!(resolved, temp_dir.path().join("wow/logger/logs"));
    }

    #[test]
    fn build_log_file_path_uses_account_and_realm_from_config() {
        let temp_dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp_dir.path().join("wow/wotlk")).expect("create wow config dir");
        fs::write(
            temp_dir.path().join("wow/wotlk/connection.toml"),
            r#"
[connection]
host = "127.0.0.1:3724"
account_name = "Admin User"
password = "secret"

[autoselect]
realm_name = "Fun Realm"
character_name = "TestChar"
"#,
        ).expect("write connection");
        let _guard = EnvGuard::set("TENTACLI_CONFIG_DIR", temp_dir.path());

        let suffix = Logger::resolve_log_file_suffix();
        let logs_dir = temp_dir.path().join("logs");
        let path = Logger::build_log_file_path(&logs_dir, &suffix);

        assert_eq!(path.parent(), Some(logs_dir.as_path()));
        let file_name = path.file_name().and_then(|x| x.to_str()).expect("file name");
        assert!(file_name.ends_with(".log"));
        assert!(regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}_").expect("regex").is_match(file_name));
        assert!(file_name.contains("Admin_User"));
        assert!(file_name.contains("Fun_Realm"));
    }

    #[test]
    fn build_fragment_file_path_appends_fragment_index() {
        let base = Path::new("logs/2026-04-28_Admin_Realm.log");

        assert_eq!(
            Logger::build_fragment_file_path(base, 0),
            Path::new("logs/2026-04-28_Admin_Realm.log")
        );
        assert_eq!(
            Logger::build_fragment_file_path(base, 2),
            Path::new("logs/2026-04-28_Admin_Realm_2.log")
        );
    }

    #[test]
    fn format_entry_contains_mangos_sections() {
        let mut packet = Packet::default();
        packet.set_type(PacketType::Incoming);
        packet.set_opcode(PacketOpcode::U16(0x01EC));
        packet.set_packet_name("SMSG_AUTH_CHALLENGE".to_string());
        packet.set_body(vec![0x01, 0x02, 0xA0]);

        let line = String::from_utf8(Logger::format_entry(&packet, 8085).expect("entry"))
            .expect("utf8 entry");

        assert!(line.contains("SERVER:"));
        assert!(line.contains("SOCKET: 127.0.0.1:8085"));
        assert!(line.contains("LENGTH: 3"));
        assert!(line.contains("OPCODE: SMSG_AUTH_CHALLENGE (0x01EC)"));
        assert!(line.contains("01 02 A0"));
    }

    #[test]
    fn format_entry_skips_unsupported_opcode_types() {
        let mut packet = Packet::default();
        packet.set_opcode(PacketOpcode::U64(0x01EC));
        packet.set_body(vec![0x01, 0x02]);

        assert!(Logger::format_entry(&packet, 8085).is_none());
    }

    #[test]
    fn parse_logger_config_default_max_file_size() {
        let cfg: Config = ConfigParser::parse_from_string(
            "logs_dir = \"wow/logger/logs\"\n".to_string()
        )
        .expect("parse config");

        assert_eq!(cfg.enabled, 0);
        assert_eq!(cfg.max_file_size, 0);
    }
}
