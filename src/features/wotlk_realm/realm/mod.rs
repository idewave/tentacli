use std::collections::BTreeMap;
use tentacli_traits::{Processor};
use tentacli_traits::types::{ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod join_channels;
mod parse_motd;
mod ping;
mod ready_for_account_data_times;
mod realm_split;
mod request_characters;
mod set_in_world;
mod set_time_speed;
mod weather;

pub struct RealmProcessor;

impl Processor for RealmProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_ADDON_INFO => {
                vec![]
            },
            Opcode::SMSG_CLIENTCACHE_VERSION => {
                vec![]
            },
            Opcode::SMSG_TUTORIAL_FLAGS => {
                vec![]
            },
            Opcode::SMSG_CHAR_ENUM => {
                vec![]
            },
            Opcode::SMSG_ACCOUNT_DATA_TIMES => {
                vec![]
            },
            Opcode::SMSG_REALM_SPLIT => {
                vec![]
            },
            Opcode::SMSG_LOGIN_SETTIMESPEED => {
                vec![Box::new(set_time_speed::Handler)]
            },
            Opcode::SMSG_SET_FORCED_REACTIONS => {
                vec![]
            },
            Opcode::SMSG_LOGOUT_COMPLETE => {
                vec![]
            },
            Opcode::SMSG_WEATHER => {
                vec![Box::new(weather::Handler)]
            },
            _ => {
                vec![]
            },
        };

        handlers
    }

    fn get_one_time_handler_map() -> BTreeMap<u16, ProcessorResult> {
        let mut handlers_map: BTreeMap<u16, ProcessorResult> = BTreeMap::new();

        // handlers_map.insert(Opcode::SMSG_AUTH_CHALLENGE, vec![
        //     Box::new(auth_challenge::Handler),
        // ]);
        handlers_map.insert(Opcode::SMSG_AUTH_RESPONSE, vec![
            Box::new(ready_for_account_data_times::Handler),
            Box::new(request_characters::Handler),
            Box::new(realm_split::Handler),
        ]);
        handlers_map.insert(Opcode::SMSG_LOGIN_VERIFY_WORLD, vec![
            Box::new(join_channels::Handler),
            Box::new(set_in_world::Handler),
        ]);
        handlers_map.insert(Opcode::SMSG_MOTD, vec![
            Box::new(parse_motd::Handler),
        ]);

        handlers_map
    }
}