use std::sync::mpsc::Sender;

use crate::ipc::pipe::types::{IncomeMessageType, LoggerOutput};

#[derive(Clone, Debug)]
pub struct MessageIncome {
    _sender: Sender<IncomeMessageType>,
}

impl MessageIncome {
    pub fn new(sender: Sender<IncomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_debug_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(IncomeMessageType::Message(LoggerOutput::Debug(message, details))).unwrap();
    }

    pub fn send_success_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(IncomeMessageType::Message(LoggerOutput::Success(message, details))).unwrap();
    }

    pub fn send_server_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(IncomeMessageType::Message(LoggerOutput::Server(message, details))).unwrap();
    }

    pub fn send_client_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(IncomeMessageType::Message(LoggerOutput::Client(message, details))).unwrap();
    }

    pub fn send_error_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(IncomeMessageType::Message(LoggerOutput::Error(message, details))).unwrap();
    }
}