use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::spell::{CooldownInfo, Spell};

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
struct Income {
    skip: u8,
    spell_count: u16,
    #[depends_on(spell_count)]
    spells: Vec<Spell>,
    cooldown_count: u16,
    #[depends_on(cooldown_count)]
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