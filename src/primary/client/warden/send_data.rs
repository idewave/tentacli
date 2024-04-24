use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::{Opcode, WardenOpcode};
use tentacli_traits::types::warden::WardenModuleInfo;

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
struct OpcodeIncome {
    opcode: u8,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
struct ModuleUseIncome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    module_md5: [u8; 16],
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    module_decrypt_key: [u8; 16],
    compressed_size: u32,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
struct ModuleCacheIncome {
    partial_size: u16,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    #[depends_on(partial_size)]
    partial: Vec<u8>,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
struct HashRequestIncome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    seed: [u8; 16],
}

// @world_opcode(Opcode::CMSG_WARDEN_DATA)
#[derive(WorldPacket, Serialize, Deserialize, Debug)]
struct Outcome {
    warden_opcode: u8,
}

// I did this part mostly according to https://www.getmangos.eu/forums/topic/3409-warden/
// unfortunately this topic incomplete and seems like TS will not finish it. In case somebody know
// how to finish please help me do this
pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (OpcodeIncome { opcode }, json) = OpcodeIncome::from_binary(&input.data)?;
        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        return match opcode {
            WardenOpcode::WARDEN_SMSG_MODULE_USE => {
                let (ModuleUseIncome {
                    module_md5,
                    module_decrypt_key,
                    compressed_size
                }, json) = ModuleUseIncome::from_binary(&input.data)?;

                response.push(HandlerOutput::ResponseMessage(
                    Opcode::get_opcode_name(input.opcode as u32)
                        .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
                    Some(json),
                ));

                let module_info = WardenModuleInfo::new(
                    module_md5.to_vec(),
                    module_decrypt_key.to_vec(),
                    compressed_size
                );

                input.session.lock().await.warden_module_info = Some(module_info);

                Ok(vec![
                    HandlerOutput::Data(Outcome {
                        warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_OK,
                    }.unpack_with_client_opcode(Opcode::CMSG_WARDEN_DATA)?),
                ])
            },
            WardenOpcode::WARDEN_SMSG_MODULE_CACHE => {
                let (ModuleCacheIncome {
                    partial,
                    ..
                }, json) = ModuleCacheIncome::from_binary(&input.data)?;

                response.push(HandlerOutput::DebugMessage(
                    Opcode::get_opcode_name(input.opcode as u32)
                        .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
                    Some(json),
                ));

                if let Some(module_info) = input.session.lock().await.warden_module_info.as_mut() {
                    module_info.add_binary(partial);

                    if module_info.loaded() {
                        // for now I do not know how to run this module,
                        // if somebody can help I would be appreciate it
                        module_info.assemble();

                        response.push(HandlerOutput::Data(Outcome {
                            warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_OK,
                        }.unpack_with_client_opcode(Opcode::CMSG_WARDEN_DATA)?));

                        return Ok(response);
                    }
                }

                Ok(vec![])
            },
            WardenOpcode::WARDEN_SMSG_HASH_REQUEST => {
                if let Some(module_info) = input.session.lock().await.warden_module_info.as_mut() {
                    let (HashRequestIncome {
                        seed
                    }, json) = HashRequestIncome::from_binary(&input.data)?;

                    response.push(HandlerOutput::DebugMessage(
                        Opcode::get_opcode_name(input.opcode as u32)
                            .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
                        Some(json),
                    ));

                    module_info.set_seed(seed.to_vec());

                    return Ok(response);
                }

                Ok(response)
            },
            _ => {
                Ok(response)
            }
        }
    }
}