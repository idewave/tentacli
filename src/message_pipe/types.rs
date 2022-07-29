use crate::client::{Character, Realm};

#[derive(Debug)]
pub enum LoggerOutput {
    Info(String),
    Debug(String),
    Error(String),
    Success(String),

    Server(String),
    Client(String),
}

pub enum MessageType {
    ChooseRealm(Vec<Realm>),
    ChooseCharacter(Vec<Character>),
    Message(LoggerOutput),
}