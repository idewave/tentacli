use std::collections::BTreeMap;

use tentacli_traits::Processor;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::ProcessorResult;

mod handle_name_query_response;
mod get_characters_list;
mod player_login;
mod check_character_create_status;
mod traits;
mod init_world_states;

pub struct PlayerProcessor;

impl Processor for PlayerProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_GROUP_INVITE => {
                vec![]
            }
            Opcode::SMSG_NAME_QUERY_RESPONSE => {
                vec![
                    Box::new(handle_name_query_response::Handler),
                ]
            }
            Opcode::SMSG_SET_PCT_SPELL_MODIFIER => {
                vec![]
            }
            Opcode::SMSG_TALENT_UPDATE => {
                vec![]
            }
            Opcode::MSG_SET_DUNGEON_DIFFICULTY => {
                vec![]
            }
            Opcode::SMSG_QUESTGIVER_STATUS_MULTIPLE => {
                vec![]
            }
            Opcode::SMSG_ACHIEVEMENT_EARNED => {
                vec![]
            }
            Opcode::SMSG_INIT_WORLD_STATES => {
                vec![Box::new(init_world_states::Handler)]
            }
            Opcode::SMSG_CHAR_CREATE => {
                vec![
                    Box::new(check_character_create_status::Handler),
                ]
            }
            _ => vec![],
        };

        handlers
    }

    fn get_one_time_handler_map() -> BTreeMap<u16, ProcessorResult> {
        let mut handlers_map: BTreeMap<u16, ProcessorResult> = BTreeMap::new();

        handlers_map.insert(Opcode::SMSG_CHAR_ENUM, vec![
            Box::new(get_characters_list::Handler),
            Box::new(player_login::Handler),
        ]);

        handlers_map
    }
}

pub mod packet {
    use serde::Serialize;

    // Opcode::CMSG_CHAR_CREATE
    #[derive(WorldPacket, Serialize, Debug)]
    pub struct CharCreateOutgoing {
        pub name: String,
        pub race: u8,
        pub class: u8,
        pub gender: u8,
        pub skin: u8,
        pub face: u8,
        pub hair_style: u8,
        pub hair_color: u8,
        pub facial_hair: u8,
        pub outfit_id: u8,
    }
}
