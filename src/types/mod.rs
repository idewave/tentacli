use std::collections::VecDeque;
use std::io::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::data_storage::DataStorage;
use crate::message_pipe::message_sender::MessageSender;
use crate::network::session::Session;

pub enum State {
    SetEncryption(Vec<u8>),
    SetConnectedToRealm(bool),
}

pub struct HandlerInput<'a> {
    pub session: &'a mut Session,
    pub data: Option<&'a [u8]>,
    pub data_storage: &'a mut DataStorage,
    pub message_sender: MessageSender,
}

pub enum HandlerOutput {
    Data((u32, Vec<u8>, Vec<u8>)),
    ConnectionRequest(String, u16),
    UpdateState(State),
    WaitForInput,
    Void,
}

pub struct AIManagerInput {
    pub session: Arc<Mutex<Session>>,
    pub data_storage: Arc<Mutex<DataStorage>>,
    pub output_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type HandlerFunction = Box<dyn FnMut(&mut HandlerInput) -> HandlerResult + Send>;

pub type ProcessorResult = Vec<HandlerFunction>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;