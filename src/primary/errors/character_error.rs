#[derive(Error, Debug)]
pub enum CharacterListError {
    #[error("Characters list is empty. Cannot select any character here.")]
    Empty,
    #[error("No realm found ! Check the Config.yml for autoselect - realm_name")]
    NotFound,
}