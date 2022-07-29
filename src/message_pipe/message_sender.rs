use std::sync::mpsc::Sender;

use crate::message_pipe::types::{LoggerOutput, MessageType};

#[derive(Clone)]
pub struct MessageSender {
    _sender: Sender<MessageType>,
}

impl MessageSender {
    pub fn new(sender: Sender<MessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_info_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Info(message))).unwrap();
    }

    pub fn send_debug_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Debug(message))).unwrap();
    }

    pub fn send_success_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Success(message))).unwrap();
    }

    pub fn send_server_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Server(message))).unwrap();
    }

    pub fn send_client_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Client(message))).unwrap();
    }

    pub fn send_error_message(&mut self, message: String) {
        self._sender.send(MessageType::Message(LoggerOutput::Error(message))).unwrap();
    }
}