use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use async_trait::async_trait;

use crate::packet;
use crate::client::opcodes::Opcode;
use crate::client::WardenModuleInfo;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;
use super::opcodes::WardenOpcode;

packet! {
    @option[world_opcode=Opcode::CMSG_WARDEN_DATA]
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
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());
        let opcode = reader.read_u8()?;

        return match opcode {
            WardenOpcode::WARDEN_SMSG_MODULE_USE => {
                let mut module_md5 = [0u8; 16];
                reader.read_exact(&mut module_md5)?;

                let mut module_decrypt_key = [0u8; 16];
                reader.read_exact(&mut module_decrypt_key)?;

                let compressed_size = reader.read_u32::<LittleEndian>()?;

                let module_info = WardenModuleInfo::new(
                    module_md5.to_vec(),
                    module_decrypt_key.to_vec(),
                    compressed_size
                );
                input.session.lock().unwrap().warden_module_info = Some(module_info);

                Ok(HandlerOutput::Data(Outcome {
                    warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_MISSING,
                }.unpack()))
            },
            WardenOpcode::WARDEN_SMSG_MODULE_CACHE => {
                if let Some(module_info) = input.session.lock().unwrap().warden_module_info.as_mut() {
                    let partial_size = reader.read_u16::<LittleEndian>()?;

                    let mut partial = vec![0u8; partial_size as usize];
                    reader.read_exact(&mut partial)?;

                    module_info.add_binary(partial);

                    if module_info.loaded() {
                        // for now I do not know how to run this module,
                        // if somebody can help I would be appreciate it
                        module_info.assemble();

                        return Ok(HandlerOutput::Data(Outcome {
                            warden_opcode: WardenOpcode::WARDEN_CMSG_MODULE_OK,
                        }.unpack()));
                    }
                }

                Ok(HandlerOutput::Void)
            },
            WardenOpcode::WARDEN_SMSG_HASH_REQUEST => {
                if let Some(module_info) = input.session.lock().unwrap().warden_module_info.as_mut() {
                    let mut seed = vec![0u8; 16];
                    reader.read_exact(&mut seed)?;

                    module_info.set_seed(seed);
                }

                Ok(HandlerOutput::Void)
            },
            _ => {
                Ok(HandlerOutput::Void)
            }
        }
    }
}