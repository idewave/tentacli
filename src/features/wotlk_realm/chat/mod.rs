use tentacli_traits::Processor;
use tentacli_traits::types::{ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod log_chat_message;
mod query_unknown_player;

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
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