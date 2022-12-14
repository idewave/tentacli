use async_trait::async_trait;
use std::io::BufRead;

use crate::client::chat::types::{MessageType};
use crate::client::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize)]
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

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (Income { .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        input.message_income.send_server_message(
            format!(
                "{} ({})",
                Opcode::get_server_opcode_name(input.opcode.unwrap()),
                "log_chat_message"
            ),
            Some(json),
        );

        Ok(HandlerOutput::Void)
    }
}