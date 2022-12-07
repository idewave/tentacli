use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

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

impl<'de> Deserialize<'de> for Realm {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for Realm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 8;
        let mut state = serializer.serialize_struct("Realm", FIELDS_AMOUNT)?;
        state.serialize_field("icon", &self.icon)?;
        state.serialize_field("flags", &self.flags)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("population", &self.population)?;
        state.serialize_field("characters", &self.characters)?;
        state.serialize_field("timezone", &self.timezone)?;
        state.serialize_field("server_id", &self.server_id)?;
        state.end()
    }
}