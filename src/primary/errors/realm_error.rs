#[derive(Error, Debug)]
pub enum RealmListError {
    #[error("No realm found ! Check the Config.yml for autoselect - realm_name")]
    NotFound,
}