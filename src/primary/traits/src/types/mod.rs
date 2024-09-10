use anyhow::{Result as AnyResult};
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::Mutex;

pub mod auth;
pub mod chat;
pub mod config;
pub mod custom_fields;
pub mod opcodes;
pub mod movement;
pub mod player;
pub mod position;
pub mod realm;
pub mod shared;
pub mod spell;
pub mod trade;
pub mod update_data;
pub mod update_fields;
pub mod warden;
pub mod world;
pub mod errors;
pub mod object;

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

/// HandlerOutput represents messages for exchanging data between core and features.
/// `HandlerOutput::ChatMessage` notifies about messages received in chat
/// `HandlerOutput::Data` is used to send some packet to server, contains of opcode, packet body and extra string details.
/// `TransferCharactersList` or `TransferRealmsList` notifies about parsed Characters/Realms list
/// (triggered by `SMSG_UPDATE_OBJECT`/`SMSG_COMPRESSED_UPDATE_OBJECT` packets)
/// `HandlerOutput::ConnectionRequest` is used to set connection (each call will **replace** current connection)
/// `HandlerOutput::Freeze` is mostly used to stop packet handling (to wait for user actions etc)
/// `HandlerOutput::Drop` exits the handle_output task (since version v4.0.0 this behavior is wrong, should be fixed in future)
/// `SelectRealm` and `SelectCharacter` are used to notify core about selection (should be used in features)
/// messages are used to notify features about output (see features/ui and features/console how they processes the output)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HandlerOutput {
    // data transfer
    ChatMessage(Message),
    Data((u32, Vec<u8>, String)),
    TransferCharactersList(Vec<Player>),
    TransferRealmsList(Vec<Realm>),
    IdentifyMe(u64),
    UpdatePlayer(u64),
    UpdateNPC(u64),
    UpdateItem(u64),
    UpdateContainer(u64),
    UpdateCorpse(u64),
    UpdateGameObject(u64),
    UpdateDynamicObject(u64),

    // commands
    ConnectionRequest(String, u16),
    Drop,
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

pub type ProcessorFunction = Box<dyn Fn(u16) -> ProcessorResult + Send>;

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
