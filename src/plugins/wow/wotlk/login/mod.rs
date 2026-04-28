use async_trait::async_trait;

mod auth;
mod network;
mod srp;

use crate::client::{
    BytesRead, ConfigParser, NetworkPlugin, OutputBuilder, Processor, ProcessorPlugin, Protocol,
    Serializer, ServerLabel,
};
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::login::auth::{AuthProcessor, login_challenge};
use crate::plugins::wow::wotlk::login::network::{PacketReader, PacketSerializer};
pub use auth::select_realm::ServerId;
pub use auth::validate_proof::Secret;

const PLUGIN_LABEL: &str = "login";

#[derive(Default)]
pub struct LoginPlugin;
#[async_trait]
impl NetworkPlugin for LoginPlugin {
    fn get_builders(&self) -> Vec<Box<dyn OutputBuilder>> {
        vec![Box::new(login_challenge::Builder)]
    }

    fn get_reader(&self) -> Box<dyn BytesRead> {
        Box::new(PacketReader)
    }

    fn get_serializer(&self) -> Box<dyn Serializer> {
        Box::new(PacketSerializer)
    }

    fn label(&self) -> ServerLabel {
        PLUGIN_LABEL
    }

    fn protocol(&self) -> Protocol {
        Protocol::TCP
    }

    fn remote_addr(&self) -> anyhow::Result<Option<String>> {
        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        if let Some(connection) = config.connection {
            Ok(Some(connection.host))
        } else {
            Ok(None)
        }
    }
}

#[derive(Default)]
pub struct Processors;
impl ProcessorPlugin for Processors {
    fn get_processors(&self) -> Vec<Box<dyn Processor>> {
        vec![Box::new(AuthProcessor::default())]
    }

    fn label(&self) -> ServerLabel {
        PLUGIN_LABEL
    }
}
