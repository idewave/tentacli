use std::sync::mpsc::Sender;

use crate::client::{Character, Realm};
use crate::ipc::pipe::types::{IncomeMessageType, OutcomeMessageType};

#[derive(Clone, Debug)]
pub struct DialogIncome {
    _sender: Sender<IncomeMessageType>,
}

impl DialogIncome {
    pub fn new(sender: Sender<IncomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_realm_dialog(&mut self, items: Vec<Realm>) {
        self._sender.send(IncomeMessageType::ChooseRealm(items)).unwrap();
    }

    pub fn send_characters_dialog(&mut self, items: Vec<Character>) {
        self._sender.send(IncomeMessageType::ChooseCharacter(items)).unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct DialogOutcome {
    _sender: Sender<OutcomeMessageType>,
}

impl DialogOutcome {
    pub fn new(sender: Sender<OutcomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_selected_realm(&mut self, item: Realm) {
        self._sender.send(OutcomeMessageType::RealmSelected(item)).unwrap();
    }

    pub fn send_selected_character(&mut self, item: Character) {
        self._sender.send(OutcomeMessageType::CharacterSelected(item)).unwrap();
    }
}