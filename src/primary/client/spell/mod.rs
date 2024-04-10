use tentacli_traits::Processor;
use tentacli_traits::types::{HandlerInput, ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod handle_initial_spells;
mod handle_spell_go;

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