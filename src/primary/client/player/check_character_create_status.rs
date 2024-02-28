use async_trait::async_trait;

use crate::packet::custom_fields::TerminatedString;
use crate::packet::player::CharCreateOutcome;
use crate::primary::client::Opcode;
use crate::primary::client::player::globals::CharacterEnumOutcome;
use crate::primary::client::player::traits::CharacterCreateToolkit;
use crate::primary::client::player::types::CharacterCreateResponseCode;
use crate::primary::traits::packet_handler::PacketHandler;
use crate::primary::types::{HandlerInput, HandlerResult};
use crate::types::HandlerOutput;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    code: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { code, .. }, json) = Income::from_binary(&input.data)?;
        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let auto_create_character_for_new_account = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.common.auto_create_character_for_new_account
        };

        match code {
            CharacterCreateResponseCode::CHAR_CREATE_SUCCESS => {
                response.push(HandlerOutput::SuccessMessage(
                    "Character created successfully".into(),
                    None,
                ));
            },
            CharacterCreateResponseCode::CHAR_CREATE_NAME_IN_USE => {
                if auto_create_character_for_new_account {
                    let random_name = Self::generate_random_string(true);
                    response.push(HandlerOutput::ResponseMessage(
                        format!("Creating character with name \"{}\"", random_name),
                        None,
                    ));

                    response.push(HandlerOutput::Data(CharCreateOutcome {
                        name: TerminatedString::from(random_name),
                        race: Self::get_random_race(),
                        class: Self::get_random_class(),
                        gender: Self::get_random_gender(),
                        skin: 0,
                        face: 0,
                        hair_style: 0,
                        hair_color: 0,
                        facial_hair: 0,
                        outfit_id: 0,
                    }.unpack()?));

                    response.push(HandlerOutput::Data(CharacterEnumOutcome::default().unpack()?));
                }
            },
            CharacterCreateResponseCode::CHAR_CREATE_ACCOUNT_LIMIT => {
                response.push(HandlerOutput::SuccessMessage(
                    "Account limit is exceeded".into(),
                    None,
                ));
                response.push(HandlerOutput::Drop);
            },
            CharacterCreateResponseCode::CHAR_CREATE_SERVER_LIMIT => {
                response.push(HandlerOutput::SuccessMessage(
                    "Server limit is exceeded".into(),
                    None,
                ));
                response.push(HandlerOutput::Drop);
            },
            _ => {},
        }

        Ok(response)
    }
}

impl CharacterCreateToolkit for Handler {}