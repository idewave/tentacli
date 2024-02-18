mod handle_initial_spells;
mod handle_spell_go;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct SpellProcessor;

impl Processor for SpellProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = match input.opcode {
            Opcode::SMSG_SPELL_GO => {
                vec![Box::new(handle_spell_go::Handler)]
            },
            Opcode::SMSG_INITIAL_SPELLS => {
                vec![
                    Box::new(handle_initial_spells::Handler),
                ]
            },
            _ => vec![]
        };

        handlers
    }
}