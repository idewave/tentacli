use anyhow::bail;
use async_trait::async_trait;
use regex::Regex;
use tentacli_traits::{PacketHandler, RealmListError};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::realm::Realm;

#[derive(LoginPacket, Serialize, Deserialize, Debug, Default)]
#[options(with_async)]
pub struct RealmlistResponse {
    skip: [u8; 6],
    realms: Vec<Realm>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (RealmlistResponse { realms, .. }, json) = RealmlistResponse::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let autoselect_realm_name = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.connection_data.autoselect_realm_name.to_string()
        };

        if autoselect_realm_name.is_empty() {
            response.push(HandlerOutput::TransferRealmsList(realms));
            response.push(HandlerOutput::Freeze);
        } else {
            let re = Regex::new(&autoselect_realm_name).unwrap();
            if let Some(realm) = realms.into_iter().find(|item| re.is_match(&item.name[..])) {
                response.push(HandlerOutput::DebugMessage(
                    format!("Selected \"{}\" Realm", realm.name),
                    None,
                ));
                input.session.lock().await.selected_realm = Some(realm);
            } else {
                bail!(RealmListError::NotFound);
            }
        }

        Ok(response)
    }
}