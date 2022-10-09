use std::io::Error;
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::{mpsc};

pub mod traits;

use crate::ipc::pipe::dialog::DialogIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::storage::DataStorage;
use crate::ipc::session::Session;
use crate::types::traits::PacketHandler;

#[derive(Debug)]
pub struct HandlerInput<'a> {
    pub session: Arc<SyncMutex<Session>>,
    pub data: Option<&'a [u8]>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub message_income: MessageIncome,
    pub dialog_income: DialogIncome,
}

#[derive(Debug)]
pub enum HandlerOutput {
    Data((u32, Vec<u8>, Vec<u8>)),
    ConnectionRequest(String, u16),
    Freeze,
    Void,
}

#[derive(Debug)]
pub struct AIManagerInput {
    pub session: Arc<SyncMutex<Session>>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub output_sender: mpsc::Sender<Vec<u8>>,
    pub message_income: MessageIncome,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type ProcessorResult = Vec<Box<dyn PacketHandler + Send>>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;