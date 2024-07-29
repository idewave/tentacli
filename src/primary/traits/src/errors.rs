#[derive(Error, Debug)]
pub enum CharacterListError {
    #[error("Characters list is empty. Cannot select any character here.")]
    Empty,
    #[error("No realm found ! Check the config file for autoselect - realm_name")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config File is not found")]
    NotFound,
    #[error("Scan error")]
    ScanError(#[source] yaml_rust::ScanError),
}

#[derive(Error, Debug)]
pub enum FieldError {
    #[error("Read error for field of type '{1}'")]
    CannotRead(#[source] std::io::Error, String),
    #[error("Invalid string when parse field of type ('{1}')")]
    InvalidString(#[source] std::string::FromUtf8Error, String),
    #[error("Write error for field of type '{1}'")]
    CannotWrite(#[source] std::io::Error, String),
}

#[derive(Error, Debug)]
pub enum RealmListError {
    #[error("No realm found ! Check the config file for autoselect - realm_name")]
    NotFound,
}