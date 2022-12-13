use std::io::Error;
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::{mpsc::Sender};

mod fields;

pub use fields::{PackedGuid, TerminatedString};

use crate::ipc::pipe::dialog::DialogIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::storage::DataStorage;
use crate::ipc::session::Session;
use crate::traits::packet_handler::PacketHandler;

#[derive(Debug)]
pub struct HandlerInput {
    pub session: Arc<SyncMutex<Session>>,
    pub data: Option<Vec<u8>>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub message_income: MessageIncome,
    pub dialog_income: DialogIncome,
    pub opcode: Option<u16>,
}

#[derive(Debug)]
pub enum HandlerOutput {
    Data(PacketOutcome),
    ConnectionRequest(String, u16),
    Freeze,
    Void,
    Drop,
}

#[derive(Debug)]
pub struct AIManagerInput {
    pub session: Arc<SyncMutex<Session>>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub output_sender: Sender<PacketOutcome>,
    pub message_income: MessageIncome,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type ProcessorResult = Vec<Box<dyn PacketHandler + Send>>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;

pub type PacketOutcome = (u32, Vec<u8>, String);