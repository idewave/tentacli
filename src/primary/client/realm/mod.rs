use tentacli_traits::Processor;
use tentacli_traits::types::{HandlerInput, ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod auth_challenge;
mod join_channels;
mod parse_motd;
mod ping;
mod ready_for_account_data_times;
mod realm_split;
mod request_characters;
mod set_in_world;
mod logout;
mod set_time_speed;
mod weather;

pub struct RealmProcessor;

impl Processor for RealmProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = match input.opcode {
            Opcode::SMSG_AUTH_CHALLENGE => {
                vec![
                    Box::new(auth_challenge::Handler),
                ]
            },
            Opcode::SMSG_AUTH_RESPONSE => {
                vec![
                    Box::new(ready_for_account_data_times::Handler),
                    Box::new(request_characters::Handler),
                    Box::new(realm_split::Handler),
                ]
            },
            Opcode::SMSG_ADDON_INFO => {
                vec![]
            },
            Opcode::SMSG_CLIENTCACHE_VERSION => {
                vec![]
            },
            Opcode::SMSG_TUTORIAL_FLAGS => {
                vec![]
            },
            Opcode::SMSG_LOGIN_VERIFY_WORLD => {
                vec![
                    Box::new(join_channels::Handler),
                    Box::new(set_in_world::Handler),
                ]
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
            Opcode::SMSG_MOTD => {
                vec![Box::new(parse_motd::Handler)]
            },
            Opcode::SMSG_LOGIN_SETTIMESPEED => {
                vec![Box::new(set_time_speed::Handler)]
            },
            Opcode::SMSG_SET_FORCED_REACTIONS => {
                vec![]
            },
            Opcode::SMSG_LOGOUT_COMPLETE => {
                vec![Box::new(logout::Handler)]
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
}

pub mod packet {
    // Opcode::CMSG_LOGOUT_REQUEST
    #[derive(WorldPacket, Serialize, Debug, Default)]
    pub struct LogoutOutcome {}
}