use std::sync::mpsc::Sender;
use crossterm::event::KeyEvent;

use crate::ipc::pipe::types::IncomeMessageType;

#[derive(Clone)]
pub struct EventIncome {
    _sender: Sender<IncomeMessageType>,
}

impl EventIncome {
    pub fn new(sender: Sender<IncomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_key_event(&mut self, key: KeyEvent) {
        self._sender.send(IncomeMessageType::KeyEvent(key.modifiers, key.code)).unwrap();
    }

    pub fn send_resize_event(&mut self) {
        self._sender.send(IncomeMessageType::ResizeEvent).unwrap();
    }
}