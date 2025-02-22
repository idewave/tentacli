use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug)]
struct Incoming {
    skip: [u8; 5],
    sender_guid: u64,
}

#[derive(WorldPacket, Serialize, Debug)]
pub struct NameQueryOutgoing {
    pub guid: u64,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { sender_guid: guid, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let guard = input.data_storage.lock().await;

        let need_send_query = match guard.players_map.get(&guid) {
            Some(player) => player.name.is_empty(),
            _ => true
        };

        if need_send_query {
            response.push(HandlerOutput::Data(
                NameQueryOutgoing { guid }.unpack_with_client_opcode(Opcode::CMSG_NAME_QUERY)?
            ));
        }

        Ok(response)
    }
}
