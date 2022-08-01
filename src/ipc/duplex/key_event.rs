use std::sync::mpsc::Sender;
use crossterm::event::KeyEvent;

use crate::ipc::duplex::types::IncomeMessageType;

#[derive(Clone)]
pub struct KeyEventIncome {
    _sender: Sender<IncomeMessageType>,
}

impl KeyEventIncome {
    pub fn new(sender: Sender<IncomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_key_event(&mut self, key: KeyEvent) {
        self._sender.send(IncomeMessageType::KeyEvent(key.modifiers, key.code)).unwrap();
    }
}