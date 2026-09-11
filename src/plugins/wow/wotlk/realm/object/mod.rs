use std::collections::HashMap;

mod accessors;
mod aura_update;
mod aura_update_all;
mod destroy_object;
mod lifecycle;
mod monster_move;
mod name_query;
mod names;
mod player_move;
pub mod types;
mod update_object;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

pub use accessors::{GameObjectRef, PlayerRef, UnitRef};
pub use lifecycle::{ObjectLifecycle, ObjectLifecycleRegistry, ObjectRemovalReason};
pub use names::{
    CreatureIdentity, GameObjectIdentity, ItemIdentity, ObjectNameRegistry, PetIdentity,
    PlayerIdentity,
};
pub use types::packed_guid::PackedGuid;
pub use types::unit::{Class, Gender, PowerType, Race};
pub use update_object::{Object, ObjectTypeId};

/// Canonical WotLK object state stored by [`ObjectProcessor`] in [`CtxMap`].
pub type ObjectMap = HashMap<PackedGuid, Object>;

/// Returns the WotLK object state maintained by [`ObjectProcessor`].
#[inline]
pub fn objects(ctx: &CtxMap) -> Option<&ObjectMap> {
    ctx.get::<ObjectMap>()
}

/// Returns mutable access to the WotLK object state maintained by [`ObjectProcessor`].
#[inline]
pub fn objects_mut(ctx: &mut CtxMap) -> Option<&mut ObjectMap> {
    ctx.get_mut::<ObjectMap>()
}

/// Returns lifecycle metadata maintained alongside [`ObjectMap`].
#[inline]
pub fn object_lifecycle(ctx: &CtxMap) -> Option<&ObjectLifecycleRegistry> {
    ctx.get::<ObjectLifecycleRegistry>()
}

/// Returns object identity/name metadata maintained alongside [`ObjectMap`].
#[inline]
pub fn object_names(ctx: &CtxMap) -> Option<&ObjectNameRegistry> {
    ctx.get::<ObjectNameRegistry>()
}

#[derive(Default)]
pub struct ObjectProcessor;
impl Processor for ObjectProcessor {
    fn init(&mut self, ctx: &mut CtxMap) -> anyhow::Result<()> {
        ctx.insert(ObjectMap::with_capacity(4_096));
        ctx.insert(ObjectLifecycleRegistry::with_capacity(4_096));
        ctx.insert(ObjectNameRegistry::default());
        Ok(())
    }

    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;
        Ok(match opcode {
            Opcode::SMSG_UPDATE_OBJECT => vec![Box::new(update_object::Handler {
                is_compressed: false,
            })],
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => vec![Box::new(update_object::Handler {
                is_compressed: true,
            })],
            Opcode::SMSG_DESTROY_OBJECT => vec![Box::new(destroy_object::Handler)],
            Opcode::SMSG_NAME_QUERY_RESPONSE => vec![Box::new(name_query::Handler {
                kind: name_query::ResponseKind::Player,
            })],
            Opcode::SMSG_PET_NAME_QUERY_RESPONSE => vec![Box::new(name_query::Handler {
                kind: name_query::ResponseKind::Pet,
            })],
            Opcode::SMSG_CREATURE_QUERY_RESPONSE => vec![Box::new(name_query::Handler {
                kind: name_query::ResponseKind::Creature,
            })],
            Opcode::SMSG_GAMEOBJECT_QUERY_RESPONSE => vec![Box::new(name_query::Handler {
                kind: name_query::ResponseKind::GameObject,
            })],
            Opcode::SMSG_ITEM_NAME_QUERY_RESPONSE => vec![Box::new(name_query::Handler {
                kind: name_query::ResponseKind::Item,
            })],
            Opcode::MSG_MOVE_START_FORWARD
            | Opcode::MSG_MOVE_START_BACKWARD
            | Opcode::MSG_MOVE_START_STRAFE_RIGHT
            | Opcode::MSG_MOVE_START_STRAFE_LEFT
            | Opcode::MSG_MOVE_JUMP
            | Opcode::MSG_MOVE_HEARTBEAT
            | Opcode::MSG_MOVE_START_TURN_LEFT
            | Opcode::MSG_MOVE_START_TURN_RIGHT
            | Opcode::MSG_MOVE_STOP
            | Opcode::MSG_MOVE_STOP_STRAFE
            | Opcode::MSG_MOVE_STOP_TURN
            | Opcode::MSG_MOVE_START_PITCH_UP
            | Opcode::MSG_MOVE_START_PITCH_DOWN
            | Opcode::MSG_MOVE_STOP_PITCH
            | Opcode::MSG_MOVE_FALL_LAND
            | Opcode::MSG_MOVE_SET_PITCH
            | Opcode::MSG_MOVE_START_SWIM
            | Opcode::MSG_MOVE_STOP_SWIM
            | Opcode::MSG_MOVE_SET_FACING => vec![Box::new(player_move::Handler)],
            Opcode::SMSG_MONSTER_MOVE => vec![Box::new(monster_move::Handler)],
            Opcode::SMSG_AURA_UPDATE_ALL => vec![Box::new(aura_update_all::Handler)],
            Opcode::SMSG_AURA_UPDATE => vec![Box::new(aura_update::Handler)],
            _ => vec![],
        })
    }
}
