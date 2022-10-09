use std::io::{BufRead, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use async_trait::async_trait;

use crate::client::realm::types::Realm;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut realms: Vec<Realm> = Vec::new();

        let mut reader = Cursor::new(input.data.as_ref().unwrap()[7..].to_vec());

        let realms_count = reader.read_i16::<LittleEndian>().unwrap();

        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = reader.read_u16::<LittleEndian>().unwrap();
            let flags = reader.read_u8().unwrap();

            reader.read_until(0, &mut name).unwrap();
            reader.read_until(0, &mut address).unwrap();

            let population = reader.read_f32::<LittleEndian>().unwrap();
            let characters = reader.read_u8().unwrap();
            let timezone = reader.read_u8().unwrap();
            let server_id = reader.read_u8().unwrap();

            realms.push(Realm {
                icon,
                flags,
                name: String::from_utf8(name[..(name.len() - 1) as usize].to_owned()).unwrap(),
                address: String::from_utf8(address[..(address.len() - 1) as usize].to_owned()).unwrap(),
                population,
                characters,
                timezone,
                server_id,
            });
        }

        input.dialog_income.send_realm_dialog(realms);

        Ok(HandlerOutput::Freeze)
    }
}