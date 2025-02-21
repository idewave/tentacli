use anyhow::bail;
use async_trait::async_trait;
use tentacli_traits::{CharacterListError, PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug)]
struct Outgoing {
    guid: u64,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (my_guid, enable_auto_create) = {
            let guard = input.session.lock().await;
            let my_guid = guard.my_guid;
            let enable_auto_create = guard
                .get_config()?.common.auto_create_character_for_new_account;

            (my_guid, enable_auto_create)
        };

        match my_guid {
            Some(guid) => {
                Ok(vec![HandlerOutput::Data(
                    Outgoing { guid }.unpack_with_client_opcode(Opcode::CMSG_PLAYER_LOGIN)?
                )])
            }
            None if enable_auto_create => Ok(vec![]),
            None => bail!(CharacterListError::Empty),
        }
    }
}