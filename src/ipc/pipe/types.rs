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
    None,
}

// messages from UI to client
pub enum OutcomeMessageType {
    RealmSelected(Realm),
    CharacterSelected(Character),
    SetUIMode(UIModeFlags),
}

#[derive(Debug)]
pub enum LoggerOutput {
    Debug(String, Option<String>),
    Error(String, Option<String>),
    Success(String, Option<String>),
    Server(String, Option<String>),
    Client(String, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Signal {
    Reconnect,
}
