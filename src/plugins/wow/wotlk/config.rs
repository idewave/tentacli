use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub host: String,
    pub account_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Common {
    pub create_character_if_empty: bool,
}

#[derive(Debug, Deserialize)]
pub struct Autoselect {
    pub realm_name: String,
    pub character_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub name: String,
    pub version: [u8; 3],
    pub build: u16,
    pub platform: String,
    pub os: String,
    pub locale: String,
    pub my_ip: String,
    pub timezone: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: Option<Connection>,
    pub game: Option<Game>,
    pub common: Option<Common>,
    pub autoselect: Option<Autoselect>,
}
