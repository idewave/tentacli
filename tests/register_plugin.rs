use tentacli::client::packet::{BytesRead, Packet, PacketOpcode, Processor, Serializer};
use tentacli::client::{
    CorePlugin, CtxMap, Echo, NetworkPlugin, OrderedOutput, ProcessorPlugin, Protocol, ServerLabel,
    Task,
};
use tentacli::register_plugin;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

#[derive(Default)]
struct ExternalCorePlugin;

#[async_trait::async_trait]
impl CorePlugin for ExternalCorePlugin {
    fn get_tasks(
        &self,
        _broadcast_rx: async_broadcast::Receiver<OrderedOutput>,
        _echo_senders: std::sync::Arc<std::collections::HashMap<ServerLabel, Sender<Echo>>>,
        _shutdown: CancellationToken,
        _context: std::sync::Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        Ok(vec![])
    }
}

register_plugin!(ExternalCorePlugin, dyn CorePlugin);

#[derive(Default)]
struct ExternalNetworkPlugin;

impl NetworkPlugin for ExternalNetworkPlugin {
    fn get_reader(&self) -> Box<dyn BytesRead> {
        Box::new(NoopReader)
    }

    fn get_serializer(&self) -> Box<dyn Serializer> {
        Box::new(NoopSerializer)
    }

    fn label(&self) -> ServerLabel {
        "macro-hygiene-external-network"
    }

    fn protocol(&self) -> Protocol {
        Protocol::TCP
    }
}

#[derive(Default)]
struct ExternalProcessors;

impl ProcessorPlugin for ExternalProcessors {
    fn get_processors(&self) -> Vec<Box<dyn Processor>> {
        vec![Box::new(NoopProcessor)]
    }

    fn label(&self) -> ServerLabel {
        "macro-hygiene-external-network"
    }
}

register_plugin!(ExternalNetworkPlugin, dyn NetworkPlugin);
register_plugin!(ExternalProcessors, dyn ProcessorPlugin);

struct NoopReader;
impl BytesRead for NoopReader {
    fn read(&mut self, _buffer: &mut [u8], _context: &CtxMap) -> anyhow::Result<Packet> {
        Ok(Packet::default())
    }
}

struct NoopSerializer;
impl Serializer for NoopSerializer {
    fn serialize(&mut self, _packet: &Packet, _context: &CtxMap) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }
}

struct NoopProcessor;
impl Processor for NoopProcessor {
    fn get_handlers(
        &mut self,
        _opcode: &PacketOpcode,
        _context: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn tentacli::client::packet::PacketHandler>>> {
        Ok(vec![])
    }
}

#[test]
fn register_plugin_macro_works_without_importing_arc_or_plugin_loader() {
    let names: Vec<&'static str> = inventory::iter::<tentacli::client::PluginLoader<dyn CorePlugin>>
        .into_iter()
        .map(|loader| loader.name)
        .collect();

    assert!(names.contains(&"ExternalCorePlugin"));
}

#[test]
fn registered_network_plugin_is_constructed_and_has_bound_processor() {
    let network_loader = inventory::iter::<tentacli::client::PluginLoader<dyn NetworkPlugin>>
        .into_iter()
        .find(|loader| loader.name == "ExternalNetworkPlugin")
        .expect("ExternalNetworkPlugin should be registered");

    let plugin = (network_loader.load)();
    assert_eq!(plugin.label(), "macro-hygiene-external-network");

    let processors = plugin.get_processors();
    assert_eq!(processors.len(), 1, "one processor should be bound by label");
}

#[tokio::test]
async fn registered_core_plugin_returns_runnable_tasks() {
    let core_loader = inventory::iter::<tentacli::client::PluginLoader<dyn CorePlugin>>
        .into_iter()
        .find(|loader| loader.name == "ExternalCorePlugin")
        .expect("ExternalCorePlugin should be registered");

    let plugin = (core_loader.load)();
    let (_tx, rx) = async_broadcast::broadcast::<OrderedOutput>(16);

    let tasks = plugin
        .get_tasks(
            rx,
            std::sync::Arc::new(std::collections::HashMap::new()),
            CancellationToken::new(),
            std::sync::Arc::new(RwLock::new(CtxMap::new())),
        )
        .expect("core plugin should produce task list");

    assert!(
        tasks.is_empty(),
        "smoke plugin returns empty task list and still runs through trait contract"
    );
}

#[test]
fn registered_processor_plugin_is_constructed_and_returns_processors() {
    let processor_loader =
        inventory::iter::<tentacli::client::PluginLoader<dyn ProcessorPlugin>>
            .into_iter()
            .find(|loader| loader.name == "ExternalProcessors")
            .expect("ExternalProcessors should be registered");

    let plugin = (processor_loader.load)();

    assert_eq!(plugin.label(), "macro-hygiene-external-network");

    let processors = plugin.get_processors();

    assert_eq!(processors.len(), 1);
}