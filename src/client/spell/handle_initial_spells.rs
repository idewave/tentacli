use async_trait::async_trait;

use crate::client::{CooldownInfo, Opcode, Spell};
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
        let (Income {spells, ..}, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        input.message_income.send_server_message(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        );

        for spell in spells {
            input.session.lock().unwrap().spells_map.insert(spell.spell_id);
        }

        Ok(HandlerOutput::Void)
    }
}