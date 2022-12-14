#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config not found")]
    NotFound,
}