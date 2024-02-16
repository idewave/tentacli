pub mod globals;
mod log_chat_message;
mod query_unknown_player;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = match input.opcode {
            Opcode::SMSG_MESSAGECHAT => {
                vec![
                    Box::new(query_unknown_player::Handler),
                    Box::new(log_chat_message::Handler),
                ]
            },
            Opcode::SMSG_TEXT_EMOTE => {
                vec![]
            },
            _ => vec![]
        };

        handlers
    }
}

pub mod packet {
    use crate::primary::client::Opcode;
    use crate::primary::macros::with_opcode;
    use crate::primary::types::TerminatedString;

    with_opcode! {
        @world_opcode(Opcode::CMSG_MESSAGECHAT)
        #[derive(WorldPacket, Serialize, Deserialize, Debug)]
        pub struct ChatOutcome {
            pub message_type: u32,
            pub language: u32,
            pub message: TerminatedString,
        }
    }

    with_opcode! {
        @world_opcode(Opcode::CMSG_EMOTE)
        #[derive(WorldPacket, Serialize, Deserialize, Debug)]
        pub struct EmoteOutcome {
            pub emote_type: u32,
        }
    }

    with_opcode! {
        @world_opcode(Opcode::CMSG_TEXT_EMOTE)
        #[derive(WorldPacket, Serialize, Deserialize, Debug)]
        pub struct TextEmoteOutcome {
            pub text_emote_type: u32,
            pub emote_num: u32,
            pub guid: u64,
        }
    }
}