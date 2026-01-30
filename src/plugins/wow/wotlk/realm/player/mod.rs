mod learned_spell;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Default)]
pub struct PlayerProcessor;
impl Processor for PlayerProcessor {
    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode, _: &CtxMap
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;
        Ok(match opcode {
            Opcode::SMSG_LEARNED_SPELL => vec![
                Box::new(learned_spell::Handler),
            ],
            _ => vec![],
        })
    }
}