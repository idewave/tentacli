use std::io::{Cursor, Read};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::client::chat::types::{EmoteType};
use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::network::session::types::ActionFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let config = input.session.get_config();
    let bot_chat = &config.bot_chat;

    let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());

    let _message_type = reader.read_u8()?;
    let _language = reader.read_u32::<LittleEndian>()?;
    let sender_guid = reader.read_u64::<LittleEndian>()?;

    // omit last byte (null terminator)
    reader.read_u8()?;

    let _target_guid = reader.read_u64::<LittleEndian>()?;
    let size = reader.read_u32::<BigEndian>()?;

    let mut message = vec![0u8; (size + 2) as usize];
    reader.read_exact(&mut message)?;

    let message = String::from_utf8_lossy(&message);

    return match message.trim_matches(char::from(0)) {
        greet if bot_chat.greet.contains(&greet.to_string()) => {
            let mut body: Vec<u8> = Vec::new();
            body.write_u32::<LittleEndian>(EmoteType::ONESHOT_WAVE as u32)?;

            Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_EMOTE, Some(body))))
        },
        follow_invite if bot_chat.follow_invite.contains(&follow_invite.to_string()) => {
            let mut body: Vec<u8> = Vec::new();
            body.write_u64::<LittleEndian>(sender_guid)?;

            input.session.follow_target = Some(sender_guid);
            input.session.action_flags.set(ActionFlags::IS_FOLLOWING, true);

            Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_SET_SELECTION, Some(body))))
        },
        "logout" => {
            Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_LOGOUT_REQUEST, None)))
        },
        _ => Ok(HandlerOutput::Void)
    }
}