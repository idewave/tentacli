use anyhow::{Result as AnyResult};
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::Mutex;

pub mod chat;
pub mod config;
pub mod custom_fields;
pub mod movement;
pub mod opcodes;
pub mod parsed_block;
pub mod player;
pub mod position;
pub mod realm;
pub mod shared;
pub mod spell;
pub mod trade;
pub mod warden;
pub mod auth;

use chat::{Message};
use player::{Player};
use realm::Realm;

use crate::PacketHandler;
use crate::types::shared::{DataStorage, Session};

#[derive(Debug, Clone)]
pub enum Signal {
    Reconnect,
}

#[derive(Debug)]
pub struct HandlerInput {
    pub session: Arc<Mutex<Session>>,
    pub data: Vec<u8>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub opcode: u16,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HandlerOutput {
    // data transfer
    ChatMessage(Message),
    Data((u32, Vec<u8>, String)),
    TransferCharactersList(Vec<Player>),
    TransferRealmsList(Vec<Realm>),
    UpdatePlayer(Player),

    // commands
    ConnectionRequest(String, u16),
    Drop,
    ExitConfirmed,
    ExitRequest,
    Freeze,
    SelectCharacter(Player),
    SelectRealm(Realm),

    // messages
    ResponseMessage(String, Option<String>),
    RequestMessage(String, Option<String>),
    DebugMessage(String, Option<String>),
    SuccessMessage(String, Option<String>),
    ErrorMessage(String, Option<String>),
}

pub type HandlerResult = AnyResult<Vec<HandlerOutput>>;

pub type ProcessorResult = Vec<Box<dyn PacketHandler + Send>>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;

#[derive(Default, Debug, Clone)]
pub struct IncomingPacket {
    pub opcode: u16,
    pub body: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct OutgoingPacket {
    pub opcode: u32,
    pub data: Vec<u8>,
    pub json_details: String,
}