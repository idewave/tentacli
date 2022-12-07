use async_trait::async_trait;

use crate::{with_opcode};
use crate::client::chat::types::{TextEmoteType};
use crate::client::opcodes::Opcode;
use crate::client::spell::types::SpellCastTargetType;
use crate::ipc::session::types::ActionFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::traits::packet_handler::PacketHandler;

// priest initial spell for healing
const SPELL_ID: u32 = 2050;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    sender_guid: u64,
    text_emote: u32,
}

with_opcode! {
    @world_opcode(Opcode::CMSG_CAST_SPELL)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        unknown: u8,
        sender_guid: u32,
        unknown2: u8,
        cast_flags: u32,
        guid: PackedGuid,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { sender_guid, text_emote } = Income::from_binary(input.data.as_ref().unwrap());

        match text_emote {
            TextEmoteType::TEXT_HEAL_ME | TextEmoteType::TEXT_HELP_ME => {
                input.session.lock().unwrap().action_flags.set(ActionFlags::IS_CASTING, true);

                Ok(HandlerOutput::Data(Outcome {
                    unknown: 0,
                    sender_guid: SPELL_ID,
                    unknown2: 0,
                    cast_flags: SpellCastTargetType::TARGET_FLAG_UNIT,
                    guid: PackedGuid(sender_guid),
                }.unpack()))
            },
            _ => {
                Ok(HandlerOutput::Void)
            },
        }
    }
}