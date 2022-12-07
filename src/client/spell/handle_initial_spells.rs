use async_trait::async_trait;

use crate::client::{CooldownInfo, Spell};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

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
        let Income {spells, ..} = Income::from_binary(input.data.as_ref().unwrap());

        for spell in spells {
            input.session.lock().unwrap().spells_map.insert(spell.spell_id);
        }

        Ok(HandlerOutput::Void)
    }
}