use tentacli_traits::Processor;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::{HandlerInput, ProcessorResult};

mod send_data;

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