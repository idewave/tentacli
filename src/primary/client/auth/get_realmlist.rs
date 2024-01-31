use anyhow::bail;
use async_trait::async_trait;
use regex::Regex;

use crate::primary::macros::with_opcode;
use crate::primary::client::{Realm, Opcode};
use crate::primary::errors::RealmListError;
use crate::primary::traits::packet_handler::PacketHandler;
use crate::primary::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};

with_opcode! {
    @login_opcode(Opcode::REALM_LIST)
    #[derive(LoginPacket, Serialize, Deserialize, Debug, Default)]
    struct Income {
        skip: [u8; 6],
        realms: Vec<Realm>,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { realms, .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
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
            let re = Regex::new(format!(r#"{}"#, autoselect_realm_name).as_str()).unwrap();
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