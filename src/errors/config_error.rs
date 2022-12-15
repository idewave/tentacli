#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File 'Config.yml' not found")]
    NotFound,
    #[error("scan error")]
    ScanError(#[source] yaml_rust::ScanError),
}