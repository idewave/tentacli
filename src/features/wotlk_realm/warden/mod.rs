use tentacli_traits::Processor;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::{ProcessorResult};

mod send_data;

pub struct WardenProcessor;

impl Processor for WardenProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_WARDEN_DATA => {
                vec![Box::new(send_data::Handler)]
            },
            _ => vec![],
        };

        handlers
    }
}