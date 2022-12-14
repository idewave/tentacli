#[derive(Error, Debug)]
pub enum IOError {
    #[error("read error")]
    ReadError(#[source] std::io::Error),
    #[error("read error")]
    StringReadError(#[source] std::string::FromUtf8Error),
    #[error("write error")]
    WriteError(#[source] std::io::Error),
}