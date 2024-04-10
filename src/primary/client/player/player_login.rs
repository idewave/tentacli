use anyhow::bail;
use async_trait::async_trait;
use tentacli_traits::{CharacterListError, PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
struct Outcome {
    guid: u64,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let me_exists = {
            let guard = input.session.lock().await;
            guard.me.is_some()
        };

        let auto_create_character_for_new_account = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.common.auto_create_character_for_new_account
        };
        
        if !me_exists {
            if auto_create_character_for_new_account {
                return Ok(response);
            }

            bail!(CharacterListError::Empty);
        }

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        response.push(
            HandlerOutput::Data(
                Outcome { guid: my_guid }.unpack_with_opcode(Opcode::CMSG_PLAYER_LOGIN)?
            )
        );

        Ok(response)
    }
}