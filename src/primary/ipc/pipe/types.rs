#[derive(Debug)]
pub enum LoggerOutput {
    // title, formatted local time, optional details
    Debug(String, Option<String>),
    Error(String, Option<String>),
    Success(String, Option<String>),
    Response(String, Option<String>),
    Request(String, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Signal {
    Reconnect,
}
