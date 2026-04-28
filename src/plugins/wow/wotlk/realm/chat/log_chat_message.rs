use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::enum_field;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    message_type: MessageType,
    language: u32,
    sender_guid: u64,
    skip: u32,
    #[br(if(message_type == MessageType::Channel))]
    channel_name: NullTerminated<String>,
    target_guid: u64,
    message_length: u32,
    message: NullTerminated<String>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let _ = Incoming::unpack(packet)?;
        Ok(vec![])
    }
}

enum_field! {
    pub enum MessageType: u8 {
        System            = 0x00,
        Say               = 0x01,
        Party             = 0x02,
        Raid              = 0x03,
        Guild             = 0x04,
        Officer           = 0x05,
        Yell              = 0x06,
        Whisper           = 0x07,
        WhisperForeign    = 0x08,
        WhisperInform     = 0x09,
        Emote             = 0x0A,
        TextEmote         = 0x0B,
        MonsterSay        = 0x0C,
        MonsterParty      = 0x0D,
        MonsterYell       = 0x0E,
        MonsterWhisper    = 0x0F,
        MonsterEmote      = 0x10,
        Channel           = 0x11,
        ChannelJoin       = 0x12,
        ChannelLeave      = 0x13,
        ChannelList       = 0x14,
        ChannelNotice     = 0x15,
        ChannelNoticeUser = 0x16,
        Afk               = 0x17,
        Dnd               = 0x18,
        Ignored           = 0x19,
        Skill             = 0x1A,
        Loot              = 0x1B,
        Money             = 0x1C,
        Opening           = 0x1D
    }
}
