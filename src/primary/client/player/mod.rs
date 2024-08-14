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
mod init_world_states;

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
            Opcode::SMSG_INIT_WORLD_STATES => {
                vec![Box::new(init_world_states::Handler)]
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
    use serde::{Serialize, Serializer};
    use serde::ser::SerializeStruct;
    use tentacli_traits::types::custom_fields::PackedGuid;
    use tentacli_traits::types::movement::Movement;
    use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, UpdateData};

    // Opcode::CMSG_CHAR_CREATE
    #[derive(WorldPacket, Serialize, Debug)]
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

    // Opcode::SMSG_UPDATE_OBJECT
    // Opcode::SMSG_COMPRESSED_UPDATE_OBJECT
    #[derive(WorldPacket, Serialize, Debug)]
    pub struct UpdateDataIncoming {
        pub blocks_amount: u32,
        #[depends_on(blocks_amount)]
        pub blocks: Vec<Block>,
    }

    #[derive(Segment, Debug, Clone, Default)]
    pub struct Block {
        pub block_type: BlockType,
        #[conditional]
        pub guid: PackedGuid,
        #[conditional]
        pub object_type_id: ObjectTypeID,
        #[conditional]
        pub movement: Movement,
        #[conditional]
        pub update_data: UpdateData,
        #[conditional]
        pub guid_count: u32,
        #[depends_on(guid_count)]
        #[conditional]
        pub guids: Vec<PackedGuid>
    }

    impl Block {
        fn guid(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::VALUES |
                BlockType::MOVEMENT |
                BlockType::CREATE_OBJECT |
                BlockType::CREATE_OBJECT2
            )
        }

        fn object_type_id(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::CREATE_OBJECT |
                BlockType::CREATE_OBJECT2
            )
        }

        fn movement(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::MOVEMENT |
                BlockType::CREATE_OBJECT |
                BlockType::CREATE_OBJECT2
            )
        }

        fn update_data(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::VALUES |
                BlockType::CREATE_OBJECT |
                BlockType::CREATE_OBJECT2
            )
        }

        fn guid_count(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::NEAR_OBJECTS |
                BlockType::OUT_OF_RANGE_OBJECTS
            )
        }

        fn guids(instance: &mut Self) -> bool {
            matches!(
                instance.block_type.0,
                BlockType::NEAR_OBJECTS |
                BlockType::OUT_OF_RANGE_OBJECTS
            )
        }
    }

    impl Serialize for Block {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let mut fields_amount = 1;

            if self.guid.0 != 0  {
                fields_amount += 1;
            }

            if !ObjectTypeID::is_none(&self.object_type_id) {
                fields_amount += 1;
            }

            if !Movement::is_empty(&self.movement) {
                fields_amount += 1;
            }

            if !UpdateData::is_empty(&self.update_data) {
                fields_amount += 1;
            }

            if self.guid_count > 0 {
                fields_amount += 2;
            }

            let mut state = serializer.serialize_struct("Block", fields_amount)?;
            state.serialize_field("block_type", &self.block_type)?;

            if self.guid.0 != 0  {
                state.serialize_field("guid", &self.guid)?;
            }

            if !ObjectTypeID::is_none(&self.object_type_id) {
                state.serialize_field("object_type_id", &self.object_type_id)?;
            }

            if !Movement::is_empty(&self.movement) {
                state.serialize_field("movement", &self.movement)?;
            }

            if !UpdateData::is_empty(&self.update_data) {
                state.serialize_field("update_data", &self.update_data)?;
            }

            if self.guid_count > 0 {
                state.serialize_field("guid_count", &self.guid_count)?;
                state.serialize_field("guids", &self.guids)?;
            }

            state.end()
        }
    }
}
