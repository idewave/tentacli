use async_trait::async_trait;
use std::io::BufRead;

use crate::primary::client::chat::types::{MessageType};
use crate::primary::client::Opcode;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::primary::traits::packet_handler::PacketHandler;

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
            Err(err) => TerminatedString::from(format!("Cannot parse message: \"{}\"", err.to_string()))
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
        let mut response = Vec::new();

        let (Income { .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        Ok(response)
    }
}