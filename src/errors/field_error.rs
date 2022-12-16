#[derive(Error, Debug)]
pub enum FieldError {
    #[error("Read error for field of type '{1}'")]
    CannotRead(#[source] std::io::Error, String),
    #[error("Invalid string when parse field of type ('{1}')")]
    InvalidString(#[source] std::string::FromUtf8Error, String),
    #[error("Write error for field of type '{1}'")]
    CannotWrite(#[source] std::io::Error, String),
}