use std::io::{BufRead, Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use async_trait::async_trait;

use crate::client::chat::types::MessageType;

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let players_map = &mut input.data_storage.lock().unwrap().players_map;

        let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());
        let message_type = reader.read_u8()?;
        let _language = reader.read_u32::<LittleEndian>()?;

        let sender_guid = reader.read_u64::<LittleEndian>()?;

        reader.read_u32::<LittleEndian>()?;

        let mut channel_name = Vec::new();
        if message_type == MessageType::CHANNEL {
            reader.read_until(0, &mut channel_name)?;
        }

        let channel_name = match channel_name.is_empty() {
            true => String::new(),
            false => {
                String::from_utf8(
                    channel_name[..(channel_name.len() - 1) as usize].to_owned()
                ).unwrap()
            },
        };

        let _target_guid = reader.read_u64::<LittleEndian>()?;

        let size = reader.read_u32::<LittleEndian>()?;

        let mut message = vec![0u8; (size - 1) as usize];

        reader.read_exact(&mut message)?;

        let message = String::from_utf8_lossy(&message);

        let sender_name = match players_map.get(&sender_guid) {
            Some(player) => player.name.to_string(),
            None => String::from("UNKNOWN"),
        };

        input.message_income.send_debug_message(
            format!("[MSG] [{}] {} ({}): {}", channel_name, sender_name, sender_guid, &message)
        );

        Ok(HandlerOutput::Void)
    }
}