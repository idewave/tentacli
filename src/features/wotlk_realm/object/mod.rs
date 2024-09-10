use tentacli_traits::Processor;
use tentacli_traits::types::{ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod update_object;
mod destroy_object;

pub struct ObjectProcessor;

impl Processor for ObjectProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT |
            Opcode::SMSG_UPDATE_OBJECT => {
                vec![Box::new(update_object::Handler)]
            },
            Opcode::SMSG_DESTROY_OBJECT => {
                vec![Box::new(destroy_object::Handler)]
            },
            _ => vec![],
        };

        handlers
    }
}
