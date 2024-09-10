use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug)]
pub struct Incoming {
    pub guid: u64,
    pub has_anim: bool,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { guid, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let mut guard = input.data_storage.lock().unwrap();

        match guid {
            g if guard.players_map.contains_key(&g) => {
                guard.players_map.remove(&g);
            },
            g if guard.units_map.contains_key(&g) => {
                guard.units_map.remove(&g);
            },
            g if guard.game_objects_map.contains_key(&g) => {
                guard.game_objects_map.remove(&g);
            },
            g if guard.dynamic_objects_map.contains_key(&g) => {
                guard.dynamic_objects_map.remove(&g);
            },
            g if guard.items_map.contains_key(&g) => {
                guard.items_map.remove(&g);
            },
            g if guard.containers_map.contains_key(&g) => {
                guard.containers_map.remove(&g);
            },
            g if guard.corpses_map.contains_key(&g) => {
                guard.corpses_map.remove(&g);
            },
            _ => {},
        }

        Ok(response)
    }
}