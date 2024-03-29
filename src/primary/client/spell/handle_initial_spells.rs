use async_trait::async_trait;

use crate::primary::client::{CooldownInfo, Opcode, Spell};
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
#[options(no_opcode)]
struct Income {
    skip: u8,
    spells: Vec<Spell>,
    cooldowns: Vec<CooldownInfo>
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income {spells, ..}, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        for spell in spells {
            input.session.lock().await.spells_map.insert(spell.spell_id);
        }

        Ok(response)
    }
}