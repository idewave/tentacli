use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use async_trait::async_trait;

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[5..].to_vec());
        let spell_amount = reader.read_u16::<LittleEndian>()?;

        for _ in 0..spell_amount {
            let spell_id = reader.read_u32::<LittleEndian>()?;
            reader.read_u16::<LittleEndian>()?;

            input.session.lock().unwrap().spells_map.insert(spell_id);
        }

        // let cooldown_amount = reader.read_u16::<LittleEndian>()?;
        // for _ in 0..cooldown_amount {
        //     let spell_id = reader.read_u32::<LittleEndian>()?;
        //     let item_id = reader.read_u16::<LittleEndian>()?;
        //     let spell_category = reader.read_u16::<LittleEndian>()?;
        //     let cooldown_duration = reader.read_u32::<LittleEndian>()?;
        //     let cooldown_category = reader.read_u32::<LittleEndian>()?;
        // }

        Ok(HandlerOutput::Void)
    }
}