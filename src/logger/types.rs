#[derive(Debug)]
pub enum LoggerOutput {
    Info(String),
    Debug(String),
    Error(String),
    Success(String),

    Server(String),
    Client(String),
}