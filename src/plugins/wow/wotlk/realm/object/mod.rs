use std::collections::HashMap;

mod aura_update;
mod aura_update_all;
mod destroy_object;
mod monster_move;
mod player_move;
mod update_object;
pub mod types;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::update_object::Object;

#[derive(Default)]
pub struct ObjectProcessor;
impl Processor for ObjectProcessor {
    fn init(&mut self, ctx: &mut CtxMap) -> anyhow::Result<()> {
        ctx.insert(HashMap::<PackedGuid, Object>::with_capacity(4_096));
        Ok(())
    }

    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;
        Ok(match opcode {
            Opcode::SMSG_UPDATE_OBJECT => vec![
                Box::new(update_object::Handler { is_compressed: false })
            ],
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => vec![
                Box::new(update_object::Handler { is_compressed: true })
            ],
            Opcode::SMSG_DESTROY_OBJECT => vec![
                Box::new(destroy_object::Handler),
            ],
            Opcode::MSG_MOVE_START_FORWARD |
            Opcode::MSG_MOVE_START_BACKWARD |
            Opcode::MSG_MOVE_START_STRAFE_RIGHT |
            Opcode::MSG_MOVE_START_STRAFE_LEFT |
            Opcode::MSG_MOVE_JUMP |
            Opcode::MSG_MOVE_HEARTBEAT |
            Opcode::MSG_MOVE_START_TURN_LEFT |
            Opcode::MSG_MOVE_START_TURN_RIGHT |
            Opcode::MSG_MOVE_STOP |
            Opcode::MSG_MOVE_STOP_STRAFE |
            Opcode::MSG_MOVE_STOP_TURN |
            Opcode::MSG_MOVE_START_PITCH_UP |
            Opcode::MSG_MOVE_START_PITCH_DOWN |
            Opcode::MSG_MOVE_STOP_PITCH |
            Opcode::MSG_MOVE_FALL_LAND |
            Opcode::MSG_MOVE_SET_PITCH |
            Opcode::MSG_MOVE_START_SWIM |
            Opcode::MSG_MOVE_STOP_SWIM |
            Opcode::MSG_MOVE_SET_FACING => vec![
                Box::new(player_move::Handler),
            ],
            Opcode::SMSG_MONSTER_MOVE => vec![
                Box::new(monster_move::Handler),
            ],
            Opcode::SMSG_AURA_UPDATE_ALL => vec![
                Box::new(aura_update_all::Handler),
            ],
            Opcode::SMSG_AURA_UPDATE => vec![
                Box::new(aura_update::Handler),
            ],
            _ => vec![],
        })
    }
}