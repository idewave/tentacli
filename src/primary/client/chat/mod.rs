use tentacli_traits::Processor;
use tentacli_traits::types::{HandlerInput, ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

pub mod globals;
mod log_chat_message;
mod query_unknown_player;

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
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
    // Opcode::CMSG_MESSAGECHAT
    #[derive(WorldPacket, Serialize, Debug)]
    pub struct ChatOutcome {
        pub message_type: u32,
        pub language: u32,
        // string should be terminated
        pub message: String,
    }

    // Opcode::CMSG_EMOTE
    #[derive(WorldPacket, Serialize, Debug)]
    pub struct EmoteOutcome {
        pub emote_type: u32,
    }

    // Opcode::CMSG_TEXT_EMOTE
    #[derive(WorldPacket, Serialize, Debug)]
    pub struct TextEmoteOutcome {
        pub text_emote_type: u32,
        pub emote_num: u32,
        pub guid: u64,
    }
}