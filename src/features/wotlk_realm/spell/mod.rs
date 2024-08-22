use tentacli_traits::Processor;
use tentacli_traits::types::{ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod handle_initial_spells;
mod handle_spell_go;
mod aura_update_all;

pub struct SpellProcessor;

impl Processor for SpellProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_SPELL_GO => {
                vec![Box::new(handle_spell_go::Handler)]
            },
            Opcode::SMSG_INITIAL_SPELLS => {
                vec![
                    Box::new(handle_initial_spells::Handler),
                ]
            },
            Opcode::SMSG_AURA_UPDATE_ALL => {
                vec![Box::new(aura_update_all::Handler)]
            },
            _ => vec![]
        };

        handlers
    }
}