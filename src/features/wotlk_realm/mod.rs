use std::collections::BTreeMap;
use tentacli_traits::{Feature, Processor};
use tentacli_traits::types::{ProcessorFunction, ProcessorResult};

mod chat;
mod player;
mod realm;
mod spell;
mod warden;
mod globals;
mod object;

use chat::ChatProcessor;
use player::PlayerProcessor;
use realm::RealmProcessor;
use spell::SpellProcessor;
use warden::WardenProcessor;
use object::ObjectProcessor;

#[derive(Default)]
pub struct WotlkRealm;
impl Feature for WotlkRealm {
    fn get_realm_processors(&self) -> Vec<ProcessorFunction> {
        vec![
            Box::new(ChatProcessor::get_handlers),
            Box::new(ObjectProcessor::get_handlers),
            Box::new(PlayerProcessor::get_handlers),
            Box::new(RealmProcessor::get_handlers),
            Box::new(SpellProcessor::get_handlers),
            Box::new(WardenProcessor::get_handlers),
        ]
    }

    fn get_one_time_handler_maps(&self) -> Vec<BTreeMap<u16, ProcessorResult>> {
        vec![
            RealmProcessor::get_one_time_handler_map()
        ]
    }
}
