use async_trait::async_trait;

mod network;
mod rc4;
mod chat;
mod init;
mod object;
mod spell;
mod player;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::chat::ChatProcessor;
use crate::plugins::wow::wotlk::realm::init::InitProcessor;
use crate::plugins::wow::wotlk::realm::network::{PacketReader, PacketSerializer};
use crate::plugins::wow::wotlk::realm::object::ObjectProcessor;
use crate::plugins::wow::wotlk::realm::player::PlayerProcessor;
use crate::plugins::wow::wotlk::realm::spell::SpellProcessor;

pub const PLUGIN_LABEL: &str = "world";

#[derive(Default)]
pub struct RealmPlugin;
#[async_trait]
impl NetworkPlugin for RealmPlugin {
    fn get_reader(&self) -> Box<dyn BytesRead> {
        Box::new(PacketReader::default())
    }

    fn get_serializer(&self) -> Box<dyn Serializer> {
        Box::new(PacketSerializer::default())
    }

    fn label(&self) -> ServerLabel {
        PLUGIN_LABEL
    }

    fn protocol(&self) -> Protocol {
        Protocol::TCP
    }
}

#[derive(Default)]
pub struct Processors;
impl ProcessorPlugin for Processors {
    #[allow(clippy::default_constructed_unit_structs)]
    fn get_processors(&self) -> Vec<Box<dyn Processor>> {
        vec![
            Box::new(InitProcessor::default()),
            Box::new(ObjectProcessor::default()),
            Box::new(ChatProcessor::default()),
            Box::new(SpellProcessor::default()),
            Box::new(PlayerProcessor::default()),
        ]
    }

    fn label(&self) -> ServerLabel {
        PLUGIN_LABEL
    }
}
