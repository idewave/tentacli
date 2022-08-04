use crossterm::event::{KeyCode, KeyModifiers};

use crate::client::{Character, Realm};

// messages from client to UI
#[derive(Debug)]
pub enum IncomeMessageType {
    ChooseRealm(Vec<Realm>),
    ChooseCharacter(Vec<Character>),
    Message(LoggerOutput),
    KeyEvent(KeyModifiers, KeyCode),
}

// messages from UI to client
pub enum OutcomeMessageType {
    RealmSelected(Realm),
    CharacterSelected(Character),
}

#[derive(Debug)]
pub enum LoggerOutput {
    Info(String),
    Debug(String),
    Error(String),
    Success(String),

    Server(String),
    Client(String),
}
