use async_trait::async_trait;
use std::io::BufRead;

use crate::{with_opcode};
use crate::client::chat::types::{EmoteType, MessageType};
use crate::client::opcodes::Opcode;
use crate::errors::ConfigError;
use crate::ipc::session::types::{ActionFlags};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
#[allow(dead_code)]
struct Income {
    message_type: u8,
    language: u32,
    sender_guid: u64,
    skip: u32,
    #[dynamic_field]
    channel_name: TerminatedString,
    target_guid: u64,
    message_length: u32,
    #[dynamic_field]
    message: TerminatedString,
}

impl Income {
    fn message<R: BufRead>(mut reader: R, initial: &mut Self) -> TerminatedString {
        let mut buffer = vec![0u8; initial.message_length as usize];
        match reader.read_exact(&mut buffer) {
            Ok(_) => TerminatedString::from(buffer),
            _ => {
                match reader.read_to_end(&mut buffer) {
                    Ok(_) => TerminatedString::from(buffer),
                    _ => TerminatedString::from("Cannot parse message"),
                }
            }
        }
    }

    fn channel_name<R: BufRead>(mut reader: R, initial: &mut Self) -> TerminatedString {
        if initial.message_type == MessageType::CHANNEL {
            let mut buffer = Vec::new();
            reader.read_until(0, &mut buffer).unwrap();
            TerminatedString::from(buffer)
        } else {
            TerminatedString::default()
        }
    }
}

with_opcode! {
    @world_opcode(Opcode::CMSG_EMOTE)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        emote_type: u32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (Income { message, sender_guid, .. }, _) = Income::from_binary(
            input.data.as_ref().unwrap()
        )?;

        let bot_chat = {
            let guard = input.session.lock().unwrap();
            let config = guard.get_config().ok_or(ConfigError::NotFound)?;
            config.bot_chat.clone()
        };

        let TerminatedString(text) = message;

        return match text.trim_matches(char::from(0)) {
            greet if bot_chat.greet.contains(&greet.to_string()) => {
                Ok(HandlerOutput::Data(Outcome {
                    emote_type: EmoteType::ONESHOT_WAVE as u32,
                }.unpack()?))
            },
            follow_invite if bot_chat.follow_invite.contains(&follow_invite.to_string()) => {
                input.message_income.send_debug_message(format!("FOLLOW {}", &sender_guid), None);

                let mut guard = input.session.lock().unwrap();
                guard.follow_target = Some(sender_guid);
                guard.action_flags.set(ActionFlags::IS_FOLLOWING, true);

                Ok(HandlerOutput::Void)
            },
            _ => Ok(HandlerOutput::Void)
        }
    }
}