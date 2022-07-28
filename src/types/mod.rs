use std::collections::VecDeque;
use std::io::Error;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;
use crate::data_storage::DataStorage;
use crate::logger::types::LoggerOutput;

use crate::network::session::Session;

pub enum State {
    SetEncryption(Vec<u8>),
    SetConnectedToRealm(bool),
}

pub struct HandlerInput<'a> {
    pub session: &'a mut Session,
    pub data: Option<&'a [u8]>,
    pub data_storage: &'a mut DataStorage,
    pub output_sender: Sender<LoggerOutput>,
}

pub enum HandlerOutput {
    Data((u32, Vec<u8>, Vec<u8>)),
    ConnectionRequest(String, u16),
    UpdateState(State),
    Void,
}

pub struct AIManagerInput {
    pub session: Arc<Mutex<Session>>,
    pub data_storage: Arc<Mutex<DataStorage>>,
    pub output_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type HandlerFunction<'a> = Box<dyn FnMut(&mut HandlerInput) -> HandlerResult + 'a>;

pub type ProcessorResult = Vec<HandlerResult>;

pub type ProcessorFunction<'a> = Box<dyn Fn(HandlerInput) -> ProcessorResult + Send + 'a>;