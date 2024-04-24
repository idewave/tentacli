use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::chat::{Message, MessageType};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;


#[derive(WorldPacket, Serialize, Deserialize)]
#[allow(dead_code)]
struct Income {
    message_type: u8,
    language: u32,
    sender_guid: u64,
    skip: u32,
    #[depends_on(message_type)]
    #[conditional]
    channel_name: String,
    target_guid: u64,
    message_length: u32,
    #[depends_on(message_length)]
    message: String,
}

impl Income {
    fn channel_name(instance: &mut Self) -> bool {
        instance.message_type == MessageType::CHANNEL
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income {
            language,
            sender_guid,
            channel_name,
            target_guid,
            message,
            message_type,
            ..
        }, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        response.push(HandlerOutput::ChatMessage(Message {
            message_type,
            language,
            sender_guid,
            channel_name: channel_name.to_string(),
            target_guid,
            text: message.to_string(),
        }));

        Ok(response)
    }
}