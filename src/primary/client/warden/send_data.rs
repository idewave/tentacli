use std::io::{BufRead};
use async_trait::async_trait;

use crate::{with_opcode};
use crate::primary::client::opcodes::Opcode;
use crate::primary::client::WardenModuleInfo;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;
use super::opcodes::WardenOpcode;

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
#[options(no_opcode)]
struct OpcodeIncome {
    opcode: u8,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
#[options(no_opcode)]
struct ModuleUseIncome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    module_md5: [u8; 16],
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    module_decrypt_key: [u8; 16],
    compressed_size: u32,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
#[options(no_opcode)]
struct ModuleCacheIncome {
    partial_size: u16,
    #[dynamic_field]
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    partial: Vec<u8>,
}

impl ModuleCacheIncome {
    fn partial<R: BufRead>(mut reader: R, initial: &mut Self) -> Vec<u8> {
        let mut buffer = vec![0u8; initial.partial_size as usize];
        reader.read_exact(&mut buffer).unwrap();
        buffer
    }
}

#[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
#[options(no_opcode)]
struct HashRequestIncome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    seed: [u8; 16],
}

with_opcode! {
    @world_opcode(Opcode::CMSG_WARDEN_DATA)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        warden_opcode: u8,
    }
}

// I did this part mostly according to https://www.getmangos.eu/forums/topic/3409-warden/
// unfortunately this topic incomplete and seems like TS will not finish it. In case somebody know
// how to finish please help me do this
pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (OpcodeIncome { opcode }, json) = OpcodeIncome::from_binary(input.data.as_ref().unwrap())?;
        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        return match opcode {
            WardenOpcode::WARDEN_SMSG_MODULE_USE => {
                let (ModuleUseIncome {
                    module_md5,
                    module_decrypt_key,
                    compressed_size
                }, json) = ModuleUseIncome::from_binary(
                    input.data.as_ref().unwrap()
                )?;

                response.push(HandlerOutput::ResponseMessage(
                    Opcode::get_server_opcode_name(input.opcode.unwrap()),
                    Some(json),
                ));

                let module_info = WardenModuleInfo::new(
                    module_md5.to_vec(),
                    module_decrypt_key.to_vec(),
                    compressed_size
                );

                input.session.lock().unwrap().warden_module_info = Some(module_info);

                Ok(vec![
                    HandlerOutput::Data(Outcome {
                        warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_OK,
                    }.unpack()?),
                ])
            },
            WardenOpcode::WARDEN_SMSG_MODULE_CACHE => {
                let (ModuleCacheIncome { partial, .. }, json) = ModuleCacheIncome::from_binary(
                    input.data.as_ref().unwrap()
                )?;

                response.push(HandlerOutput::ResponseMessage(
                    Opcode::get_server_opcode_name(input.opcode.unwrap()),
                    Some(json),
                ));

                if let Some(module_info) = input.session.lock().unwrap().warden_module_info.as_mut() {
                    module_info.add_binary(partial);

                    if module_info.loaded() {
                        // for now I do not know how to run this module,
                        // if somebody can help I would be appreciate it
                        module_info.assemble();

                        response.push(HandlerOutput::Data(Outcome {
                            warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_OK,
                        }.unpack()?));

                        return Ok(response);
                    }
                }

                Ok(vec![])
            },
            WardenOpcode::WARDEN_SMSG_HASH_REQUEST => {
                if let Some(module_info) = input.session.lock().unwrap().warden_module_info.as_mut() {
                    let (HashRequestIncome { seed }, json) = HashRequestIncome::from_binary(
                        input.data.as_ref().unwrap()
                    )?;

                    response.push(HandlerOutput::ResponseMessage(
                        Opcode::get_server_opcode_name(input.opcode.unwrap()),
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