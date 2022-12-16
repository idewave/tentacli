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
    SetUIFlag(UIModeFlags),
}

#[derive(Debug)]
pub enum LoggerOutput {
    // title, formatted local time, optional details
    Debug(String, String, Option<String>),
    Error(String, String, Option<String>),
    Success(String, String, Option<String>),
    Server(String, String, Option<String>),
    Client(String, String, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Signal {
    Reconnect,
}
