use std::sync::Arc;

use async_trait::async_trait;
use binrw::{BinRead, BinWrite};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;
use crate::plugins::wow::wotlk::realm::object::names::ObjectNameRegistry;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_NAME_QUERY)]
#[opcode(U32(Opcode::CMSG_NAME_QUERY))]
#[bw(little)]
struct PlayerNameQuery {
    guid: u64,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_PET_NAME_QUERY)]
#[opcode(U32(Opcode::CMSG_PET_NAME_QUERY))]
#[bw(little)]
struct PetNameQuery {
    pet_number: u32,
    guid: u64,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_CREATURE_QUERY)]
#[opcode(U32(Opcode::CMSG_CREATURE_QUERY))]
#[bw(little)]
struct CreatureNameQuery {
    entry: u32,
    guid: u64,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_GAMEOBJECT_QUERY)]
#[opcode(U32(Opcode::CMSG_GAMEOBJECT_QUERY))]
#[bw(little)]
struct GameObjectNameQuery {
    entry: u32,
    guid: u64,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_ITEM_NAME_QUERY)]
#[opcode(U32(Opcode::CMSG_ITEM_NAME_QUERY))]
#[bw(little)]
struct ItemNameQuery {
    entry: u32,
    guid: u64,
}

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct PlayerResponse {
    guid: PackedGuid,
    name_unknown: u8,
    #[br(if(name_unknown == 0))]
    name: Option<NullTerminated<String>>,
    #[br(if(name_unknown == 0))]
    realm_name: Option<NullTerminated<String>>,
    #[br(if(name_unknown == 0))]
    race: Option<u8>,
    #[br(if(name_unknown == 0))]
    gender: Option<u8>,
    #[br(if(name_unknown == 0))]
    class: Option<u8>,
}

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct PetResponse {
    pet_number: u32,
    name: NullTerminated<String>,
    name_timestamp: u32,
}

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct CreatureResponse {
    entry: u32,
    #[br(if(entry & 0x8000_0000 == 0))]
    name: Option<NullTerminated<String>>,
}

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct GameObjectResponse {
    entry: u32,
    #[br(if(entry & 0x8000_0000 == 0))]
    game_object_type: Option<u32>,
    #[br(if(entry & 0x8000_0000 == 0))]
    display_id: Option<u32>,
    #[br(if(entry & 0x8000_0000 == 0))]
    name: Option<NullTerminated<String>>,
}

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct ItemResponse {
    entry: u32,
    name: NullTerminated<String>,
    inventory_type: u32,
}

pub struct Handler {
    pub kind: ResponseKind,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let request = match self.kind {
            ResponseKind::Player => player_request(packet)?,
            ResponseKind::Pet => pet_request(packet)?,
            ResponseKind::Creature => creature_request(packet)?,
            ResponseKind::GameObject => game_object_request(packet)?,
            ResponseKind::Item => item_request(packet)?,
        };

        Ok(vec![HandlerOutput::Requests(vec![request])])
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ResponseKind {
    Player,
    Pet,
    Creature,
    GameObject,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NameQueryRequest {
    Player { guid: PackedGuid },
    Pet { pet_number: u32, guid: PackedGuid },
    Creature { entry: u32, guid: PackedGuid },
    GameObject { entry: u32, guid: PackedGuid },
    Item { entry: u32, guid: PackedGuid },
}

impl NameQueryRequest {
    pub(crate) fn pack(self) -> anyhow::Result<Packet> {
        match self {
            Self::Player { guid } => PlayerNameQuery { guid: guid.0 }.pack(),
            Self::Pet { pet_number, guid } => PetNameQuery {
                pet_number,
                guid: guid.0,
            }
            .pack(),
            Self::Creature { entry, guid } => CreatureNameQuery {
                entry,
                guid: guid.0,
            }
            .pack(),
            Self::GameObject { entry, guid } => GameObjectNameQuery {
                entry,
                guid: guid.0,
            }
            .pack(),
            Self::Item { entry, guid } => ItemNameQuery {
                entry,
                guid: guid.0,
            }
            .pack(),
        }
    }
}

fn player_request(packet: &mut Packet) -> anyhow::Result<Request> {
    let incoming = PlayerResponse::unpack(packet)?;

    Ok(Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
        let Some(registry) = ctx.get_mut::<ObjectNameRegistry>() else {
            return;
        };

        if incoming.name_unknown != 0 {
            registry.fail_player(incoming.guid);
            return;
        }

        registry.resolve_player(
            incoming.guid,
            incoming.name.map(|value| value.0).unwrap_or_default(),
            incoming.realm_name.map(|value| value.0).unwrap_or_default(),
        );
    }))))
}

fn pet_request(packet: &mut Packet) -> anyhow::Result<Request> {
    let incoming = PetResponse::unpack(packet)?;

    Ok(Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
        let Some(registry) = ctx.get_mut::<ObjectNameRegistry>() else {
            return;
        };

        if incoming.name.0.is_empty() {
            registry.fail_pet(incoming.pet_number);
        } else {
            registry.resolve_pet(incoming.pet_number, incoming.name.0, incoming.name_timestamp);
        }
    }))))
}

fn creature_request(packet: &mut Packet) -> anyhow::Result<Request> {
    let incoming = CreatureResponse::unpack(packet)?;
    let failed = incoming.entry & 0x8000_0000 != 0;
    let entry = incoming.entry & 0x7fff_ffff;

    Ok(Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
        let Some(registry) = ctx.get_mut::<ObjectNameRegistry>() else {
            return;
        };

        if failed {
            registry.fail_creature(entry);
        } else if let Some(name) = incoming.name {
            registry.resolve_creature(entry, name.0);
        }
    }))))
}

fn game_object_request(packet: &mut Packet) -> anyhow::Result<Request> {
    let incoming = GameObjectResponse::unpack(packet)?;
    let failed = incoming.entry & 0x8000_0000 != 0;
    let entry = incoming.entry & 0x7fff_ffff;

    Ok(Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
        let Some(registry) = ctx.get_mut::<ObjectNameRegistry>() else {
            return;
        };

        if failed {
            registry.fail_game_object(entry);
        } else if let (Some(name), Some(game_object_type), Some(display_id)) = (
            incoming.name,
            incoming.game_object_type,
            incoming.display_id,
        ) {
            registry.resolve_game_object(entry, name.0, game_object_type, display_id);
        }
    }))))
}

fn item_request(packet: &mut Packet) -> anyhow::Result<Request> {
    let incoming = ItemResponse::unpack(packet)?;

    Ok(Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
        let Some(registry) = ctx.get_mut::<ObjectNameRegistry>() else {
            return;
        };

        registry.resolve_item(incoming.entry, incoming.name.0, incoming.inventory_type);
    }))))
}
