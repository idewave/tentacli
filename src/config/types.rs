pub struct ConnectionData {
    pub username: String,
    pub password: String,
    pub realm_name: String,
}

pub struct AddonInfo {
    pub name: String,
    pub flags: u8,
    pub modulus_crc: u32,
    pub urlcrc_crc: u32,
}

pub struct BotChat {
    pub greet: Vec<String>,
    pub agree: Vec<String>,
    pub disagree: Vec<String>,
    pub follow_invite: Vec<String>,
    pub stop: Vec<String>,
}