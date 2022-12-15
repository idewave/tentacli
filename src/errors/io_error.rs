#[derive(Error, Debug)]
pub enum IOError {
    #[error("Cannot read from field of type '{1}'")]
    CannotRead(#[source] std::io::Error, String),
    #[error("Cannot read string ('{1}')")]
    CannotReadString(#[source] std::string::FromUtf8Error, String),
    #[error("Cannot write into field of type '{1}'")]
    CannotWrite(#[source] std::io::Error, String),
}