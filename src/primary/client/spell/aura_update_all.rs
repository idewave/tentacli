use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::spell::AuraFlags;

#[derive(WorldPacket, Serialize, Debug, Clone)]
struct Incoming {
    target_guid: PackedGuid,
    #[serde(skip_serializing_if = "AuraInfo::is_default")]
    aura_info: AuraInfo,
}

#[derive(Segment, Serialize, Debug, Default, Clone, PartialEq)]
struct AuraInfo {
    aura_slot: u8,
    aura_id: u32,
    aura_flags: AuraFlags,
    aura_level: u8,
    #[conditional]
    #[serde(skip_serializing_if = "PackedGuid::is_default")]
    caster_guid: PackedGuid,
    #[conditional]
    aura_max_duration: u32,
    #[conditional]
    aura_duration: u32,
}

impl AuraInfo {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    fn caster_guid(instance: &mut Self) -> bool {
        return !instance.aura_flags.contains(AuraFlags::NOT_CASTER);
    }

    fn aura_max_duration(instance: &mut Self) -> bool {
        return instance.aura_flags.contains(AuraFlags::DURATION);
    }

    fn aura_duration(instance: &mut Self) -> bool {
        return instance.aura_flags.contains(AuraFlags::DURATION);
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        // ..

        Ok(response)
    }
}