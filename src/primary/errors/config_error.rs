#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config File is not found")]
    NotFound,
    #[error("Scan error")]
    ScanError(#[source] yaml_rust::ScanError),
}