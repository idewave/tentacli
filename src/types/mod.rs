use std::collections::VecDeque;
use std::io::Error;
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::Mutex;

pub mod traits;

use crate::ipc::pipe::dialog::DialogIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::storage::DataStorage;
use crate::ipc::session::Session;

#[derive(Debug)]
pub enum State {
    SetEncryption(Vec<u8>),
    SetConnectedToRealm(bool),
}

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
    UpdateState(State),
    Freeze,
    Void,
}

#[derive(Debug)]
pub struct AIManagerInput {
    pub session: Arc<SyncMutex<Session>>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub output_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type HandlerFunction = Box<dyn FnMut(&mut HandlerInput) -> HandlerResult + Send>;

pub type ProcessorResult = Vec<HandlerFunction>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;