use std::io::Error;
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::{mpsc};

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
}

#[derive(Debug)]
pub enum HandlerOutput {
    Data(PacketOutcome),
    ConnectionRequest(String, u16),
    Freeze,
    Void,
}

#[derive(Debug)]
pub struct AIManagerInput {
    pub session: Arc<SyncMutex<Session>>,
    pub data_storage: Arc<SyncMutex<DataStorage>>,
    pub output_sender: mpsc::Sender<PacketOutcome>,
    pub message_income: MessageIncome,
}

pub type HandlerResult = Result<HandlerOutput, Error>;

pub type ProcessorResult = Vec<Box<dyn PacketHandler + Send>>;

pub type ProcessorFunction = Box<dyn Fn(&mut HandlerInput) -> ProcessorResult + Send>;

pub type PacketOutcome = (u32, Vec<u8>);

#[derive(Debug, Default, Clone)]
pub struct PackedGuid(pub u64);

impl PartialEq<u64> for PackedGuid {
    fn eq(&self, other: &u64) -> bool {
        let PackedGuid(guid) = self;
        guid == other
    }
}

impl PartialEq<PackedGuid> for u64 {
    fn eq(&self, other: &PackedGuid) -> bool {
        let PackedGuid(guid) = other;
        guid == self
    }
}

#[derive(Debug, Default, Clone)]
pub struct TerminatedString {
    value: String,
}

impl TerminatedString {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn from(value: String) -> Self {
        Self {
            value,
        }
    }

    pub fn get_value(&mut self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Default, Clone)]
pub struct IpAddr(pub u32);