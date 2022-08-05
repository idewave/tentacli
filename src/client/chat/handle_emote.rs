use std::io::{Cursor, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::client::chat::types::{TextEmoteType};
use crate::client::opcodes::Opcode;
use crate::client::spell::types::SpellCastTargetType;
use crate::ipc::session::types::ActionFlags;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::pack_guid;

// priest initial spell for healing
const SPELL_ID: u32 = 2050;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let mut session = input.session.lock().unwrap();
    let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());

    let sender_guid = reader.read_u64::<LittleEndian>()?;
    let text_emote = reader.read_u32::<LittleEndian>()?;

    match text_emote {
        TextEmoteType::TEXT_HEAL_ME | TextEmoteType::TEXT_HELP_ME => {
            session.action_flags.set(ActionFlags::IS_CASTING, true);

            let mut body = Vec::new();
            body.write_u8(0)?;
            body.write_u32::<LittleEndian>(SPELL_ID)?;
            body.write_u8(0)?;
            body.write_u32::<LittleEndian>(SpellCastTargetType::TARGET_FLAG_UNIT)?;
            body.write_all(&pack_guid(sender_guid))?;

            Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_CAST_SPELL, Some(body))))
        },
        _ => {
            Ok(HandlerOutput::Void)
        },
    }
}