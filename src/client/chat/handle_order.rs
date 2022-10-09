use std::io::{BufRead, Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use async_trait::async_trait;

use crate::client::chat::types::{EmoteType, MessageType};
use crate::client::opcodes::Opcode;
use crate::ipc::session::types::{ActionFlags};
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let bot_chat = {
            let guard = input.session.lock().unwrap();
            let config = guard.get_config().unwrap();
            config.bot_chat.clone()
        };

        let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());

        let message_type = reader.read_u8()?;
        let _language = reader.read_u32::<LittleEndian>()?;
        let sender_guid = reader.read_u64::<LittleEndian>()?;

        reader.read_u32::<LittleEndian>()?;

        let mut channel_name = Vec::new();
        if message_type == MessageType::CHANNEL {
            reader.read_until(0, &mut channel_name)?;
        }

        let _target_guid = reader.read_u64::<LittleEndian>()?;
        let size = reader.read_u32::<LittleEndian>()?;

        let mut message = vec![0u8; (size - 1) as usize];
        reader.read_exact(&mut message)?;

        let message = String::from_utf8_lossy(&message);

        return match message.trim_matches(char::from(0)) {
            greet if bot_chat.greet.contains(&greet.to_string()) => {
                let mut body: Vec<u8> = Vec::new();
                body.write_u32::<LittleEndian>(EmoteType::ONESHOT_WAVE as u32)?;

                Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_EMOTE, Some(body))))
            },
            follow_invite if bot_chat.follow_invite.contains(&follow_invite.to_string()) => {
                input.message_income.send_debug_message(format!("FOLLOW {}", &sender_guid));

                let mut body: Vec<u8> = Vec::new();
                body.write_u64::<LittleEndian>(sender_guid)?;

                let mut guard = input.session.lock().unwrap();
                guard.follow_target = Some(sender_guid);
                guard.action_flags.set(ActionFlags::IS_FOLLOWING, true);

                // Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_SET_SELECTION, Some(body))))
                Ok(HandlerOutput::Void)
            },
            _ => Ok(HandlerOutput::Void)
        }
    }
}