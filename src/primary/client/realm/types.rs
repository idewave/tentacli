use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use async_trait::async_trait;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

use crate::errors::FieldError;
use crate::primary::traits::StreamReader;
use crate::traits::BinaryConverter;

#[derive(Clone, Default)]
pub struct Realm {
    pub icon: u8,
    pub lock: u8,
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
        state.serialize_field("lock", &self.lock)?;
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

impl BinaryConverter for Vec<Realm> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut realms = Vec::new();
        let label = "Vec<Realm>";

        let realms_count = reader.read_i16::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("realms_count:i16 ({})", label)))?;
        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("icon:u8 ({})", label)))?;
            let lock = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("lock:u8 ({})", label)))?;
            let flags = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("flags:u8 ({})", label)))?;

            reader.read_until(0, &mut name)
                .map_err(|e| FieldError::CannotRead(e, format!("name:Vec<u8> ({})", label)))?;
            reader.read_until(0, &mut address)
                .map_err(|e| FieldError::CannotRead(e, format!("address:Vec<u8> ({})", label)))?;

            let population = reader.read_f32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("population:f32 ({})", label)))?;
            let characters = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("characters:u8 ({})", label)))?;
            let timezone = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("timezone:u8 ({})", label)))?;
            let server_id = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("server_id:u8 ({})", label)))?;

            realms.push(Realm {
                icon,
                lock,
                flags,
                name: String::from_utf8_lossy(&name).trim_matches(char::from(0)).to_string(),
                address: String::from_utf8_lossy(&address).trim_matches(char::from(0)).to_string(),
                population,
                characters,
                timezone,
                server_id,
            });
        }

        Ok(realms)
    }
}

#[async_trait]
impl StreamReader for Vec<Realm> {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        let mut realms = Vec::new();
        let label = "Vec<Realm>";

        let realms_count = stream.read_i16_le().await
            .map_err(|e| FieldError::CannotRead(e, format!("realms_count:i16 ({})", label)))?;
        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("icon:u8 ({})", label)))?;
            let lock = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("lock:u8 ({})", label)))?;
            let flags = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("flags:u8 ({})", label)))?;

            stream.read_until(0, &mut name).await
                .map_err(|e| FieldError::CannotRead(e, format!("name:Vec<u8> ({})", label)))?;
            stream.read_until(0, &mut address).await
                .map_err(|e| FieldError::CannotRead(e, format!("address:Vec<u8> ({})", label)))?;

            let population = stream.read_f32_le().await
                .map_err(|e| FieldError::CannotRead(e, format!("population:f32 ({})", label)))?;
            let characters = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("characters:u8 ({})", label)))?;
            let timezone = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("timezone:u8 ({})", label)))?;
            let server_id = stream.read_u8().await
                .map_err(|e| FieldError::CannotRead(e, format!("server_id:u8 ({})", label)))?;

            realms.push(Realm {
                icon,
                lock,
                flags,
                name: String::from_utf8_lossy(&name).trim_matches(char::from(0)).to_string(),
                address: String::from_utf8_lossy(&address).trim_matches(char::from(0)).to_string(),
                population,
                characters,
                timezone,
                server_id,
            });
        }

        Ok(realms)
    }
}