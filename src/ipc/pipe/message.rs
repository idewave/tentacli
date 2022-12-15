use std::sync::mpsc::Sender;
use chrono::Local;

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
        self._sender.send(
            IncomeMessageType::Message(
                LoggerOutput::Debug(
                    message, Local::now().format("[%H:%M:%S]").to_string(), details,
                ),
            ),
        ).unwrap();
    }

    pub fn send_success_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(
            IncomeMessageType::Message(
                LoggerOutput::Success(
                    message, Local::now().format("[%H:%M:%S]").to_string(), details,
                ),
            ),
        ).unwrap();
    }

    pub fn send_server_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(
            IncomeMessageType::Message(
                LoggerOutput::Server(
                    message, Local::now().format("[%H:%M:%S]").to_string(), details,
                ),
            ),
        ).unwrap();
    }

    pub fn send_client_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(
            IncomeMessageType::Message(
                LoggerOutput::Client(
                    message, Local::now().format("[%H:%M:%S]").to_string(), details,
                ),
            ),
        ).unwrap();
    }

    pub fn send_error_message(&mut self, message: String, details: Option<String>) {
        self._sender.send(
            IncomeMessageType::Message(
                LoggerOutput::Error(
                    message, Local::now().format("[%H:%M:%S]").to_string(), details,
                ),
            ),
        ).unwrap();
    }
}