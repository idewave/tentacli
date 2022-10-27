use std::fmt::{Debug, Formatter};

#[derive(Clone, Default)]
pub struct Realm {
    pub icon: u16,
    pub flags: u8,
    pub name: String,
    pub address: String,
    pub population: f32,
    pub characters: u8,
    pub timezone: u8,
    pub server_id: u8,
}

impl Debug for Realm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nicon: {:?}, flags: {}, name: '{}' address: {:?}, server_id: {:?}\n",
            self.icon,
            self.flags,
            self.name,
            self.address,
            self.server_id,
        )
    }
}