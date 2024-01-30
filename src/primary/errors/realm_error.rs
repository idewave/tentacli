#[derive(Error, Debug)]
pub enum RealmListError {
    #[error("No realm found ! Check the config file for autoselect - realm_name")]
    NotFound,
}