mod spell_go;
mod spell_start;
mod types;
mod heal_log;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Default)]
pub struct SpellProcessor;
impl Processor for SpellProcessor {
    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;
        Ok(match opcode {
            Opcode::SMSG_SPELL_GO => vec![
                Box::new(spell_go::Handler),
            ],
            Opcode::SMSG_SPELL_START => vec![
                Box::new(spell_start::Handler),
            ],
            Opcode::SMSG_SPELLHEALLOG => vec![
                Box::new(heal_log::Handler),
            ],
            _ => vec![]
        })
    }
}