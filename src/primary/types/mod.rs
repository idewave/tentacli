use std::sync::{Arc, Mutex as SyncMutex};
use anyhow::{Result as AnyResult};
use tokio::sync::Mutex;

mod fields;

pub use fields::{PackedGuid, TerminatedString};
use crate::primary::client::{Message, Player, Realm};

use crate::primary::shared::storage::DataStorage;
use crate::primary::shared::session::Session;
use crate::primary::traits::packet_handler::PacketHandler;

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
    Data(OutcomePacket),
    TransferCharactersList(Vec<Player>),
    TransferRealmsList(Vec<Realm>),
    UpdatePlayer(Player),
    ChatMessage(Message),

    // commands
    ConnectionRequest(String, u16),
    Freeze,
    Drop,
    SelectRealm(Realm),
    SelectCharacter(Player),

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

#[derive(Default, Debug)]
pub struct IncomePacket {
    pub opcode: u16,
    pub body: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct OutcomePacket {
    pub opcode: u32,
    pub data: Vec<u8>,
    pub json_details: String,
}