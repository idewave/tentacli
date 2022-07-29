use std::sync::mpsc::Sender;

use crate::client::{Character, Realm};

use crate::message_pipe::types::MessageType;

#[derive(Clone)]
pub struct DialogSender {
    _sender: Sender<MessageType>,
}

impl DialogSender {
    pub fn new(sender: Sender<MessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_realm_dialog(&mut self, items: Vec<Realm>) {
        self._sender.send(MessageType::ChooseRealm(items)).unwrap();
    }

    pub fn send_characters_dialog(&mut self, items: Vec<Character>) {
        self._sender.send(MessageType::ChooseCharacter(items)).unwrap();
    }
}