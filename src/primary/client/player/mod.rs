use tentacli_traits::Processor;
use tentacli_traits::types::{HandlerInput, ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

pub mod globals;
mod handle_name_query_response;
mod handle_update_data;
pub mod get_characters_list;
pub mod player_login;
mod check_character_create_status;
mod traits;

pub struct PlayerProcessor;

impl Processor for PlayerProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = match input.opcode {
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT |
            Opcode::SMSG_UPDATE_OBJECT => {
                vec![
                    Box::new(handle_update_data::Handler),
                ]
            },
            Opcode::SMSG_GROUP_INVITE => {
                vec![]
            },
            Opcode::SMSG_NAME_QUERY_RESPONSE => {
                vec![
                    Box::new(handle_name_query_response::Handler),
                ]
            },
            Opcode::SMSG_SET_PCT_SPELL_MODIFIER => {
                vec![]
            },
            Opcode::SMSG_TALENT_UPDATE => {
                vec![]
            },
            Opcode::MSG_SET_DUNGEON_DIFFICULTY => {
                vec![]
            },
            Opcode::SMSG_QUESTGIVER_STATUS_MULTIPLE => {
                vec![]
            },
            Opcode::SMSG_ACHIEVEMENT_EARNED => {
                vec![]
            },
            Opcode::SMSG_CHAR_ENUM => {
                vec![
                    Box::new(get_characters_list::Handler),
                    Box::new(player_login::Handler),
                ]
            },
            Opcode::SMSG_CHAR_CREATE => {
                vec![
                    Box::new(check_character_create_status::Handler),
                ]
            },
            _ => vec![],
        };

        handlers
    }
}

pub mod packet {
    // Opcode::CMSG_CHAR_CREATE
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    pub struct CharCreateOutcome {
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
