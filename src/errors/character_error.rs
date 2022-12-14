#[derive(Error, Debug)]
pub enum CharacterListError {
    #[error("Characters list is empty. Cannot select any character here.")]
    Empty,
}