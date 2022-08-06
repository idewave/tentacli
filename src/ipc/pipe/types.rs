use crossterm::event::{KeyCode, KeyModifiers};

use crate::client::{Character, Realm};
use crate::ui::types::UIModeFlags;

// messages from client to UI
#[derive(Debug)]
pub enum IncomeMessageType {
    ChooseRealm(Vec<Realm>),
    ChooseCharacter(Vec<Character>),
    Message(LoggerOutput),
    KeyEvent(KeyModifiers, KeyCode),
    ResizeEvent,
}

// messages from UI to client
pub enum OutcomeMessageType {
    RealmSelected(Realm),
    CharacterSelected(Character),
    SetUIMode(UIModeFlags),
}

#[derive(Debug)]
pub enum LoggerOutput {
    Debug(String),
    Error(String),
    Success(String),

    Server(String),
    Client(String),
}
