mod opcodes;
mod send_data;
pub mod types;

use crate::primary::client::Opcode;
use crate::primary::traits::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct WardenProcessor;

impl Processor for WardenProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = match input.opcode {
            Opcode::SMSG_WARDEN_DATA => {
                vec![Box::new(send_data::Handler)]
            },
            _ => vec![],
        };

        handlers
    }
}