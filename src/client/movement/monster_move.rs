// use std::io::{Cursor, Read, Write};
// use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
//
// use crate::client::chat::types::{EmoteType, MessageType, TextEmoteType};
// use crate::client::opcodes::Opcode;
// use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
//
// pub fn handler(input: &mut HandlerInput) -> HandlerResult {
//     let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());
//
//     let mut body: Vec<u8> = Vec::new();
//     body.write_u32::<LittleEndian>(MessageType::CHAT_MSG_SAY as u32)?;
//     body.write_u32::<LittleEndian>(0)?;
//     body.write_all(String::from("MONSTER MOVED !").as_bytes())?;
//
//     let mut header = Vec::new();
//     header.write_u16::<BigEndian>((body.len() + 4) as u16)?;
//     header.write_u32::<LittleEndian>(Opcode::CMSG_MESSAGECHAT)?;
//
//     let mut packet = Vec::new();
//     packet.write(&header)?;
//     packet.write(&body)?;
//
//     Ok(HandlerOutput::Data(packet))
// }