use std::fmt::Debug;
use std::io::BufRead;

use async_trait::async_trait;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt};

use crate::{BinaryConverter, StreamReader};
use crate::types::errors::FieldError;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Realm {
    pub icon: u8,
    pub lock: u8,
    pub flags: u8,
    pub name: String,
    pub address: String,
    pub population: f32,
    pub characters_amount: u8,
    pub timezone: u8,
    pub server_id: u8,
}

impl BinaryConverter for Realm {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        self.icon.write_into(buffer)?;
        self.lock.write_into(buffer)?;
        self.flags.write_into(buffer)?;
        format!("{}\0", self.name).write_into(buffer)?;
        format!("{}\0", self.address).write_into(buffer)?;
        self.population.write_into(buffer)?;
        self.characters_amount.write_into(buffer)?;
        self.timezone.write_into(buffer)?;
        self.server_id.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        let label = "Realm";
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

        Ok(Realm {
            icon,
            lock,
            flags,
            name: String::from_utf8_lossy(&name).trim_matches(char::from(0)).to_string(),
            address: String::from_utf8_lossy(&address).trim_matches(char::from(0)).to_string(),
            population,
            characters_amount: characters,
            timezone,
            server_id,
        })
    }
}

#[async_trait]
impl StreamReader for Realm {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
    where
        Self: Sized,
        R: AsyncBufRead + Unpin + Send,
    {
        let label = "Realm";
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

        Ok(Realm {
            icon,
            lock,
            flags,
            name: String::from_utf8_lossy(&name).trim_matches(char::from(0)).to_string(),
            address: String::from_utf8_lossy(&address).trim_matches(char::from(0)).to_string(),
            population,
            characters_amount: characters,
            timezone,
            server_id,
        })
    }
}