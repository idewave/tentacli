use std::cell::RefCell;
use std::io::{Cursor};
use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use crate::client::movement::parsers::movement_parser::MovementParser;
use crate::client::{Opcode};
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;
use crate::utils::{read_packed_guid};

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[4..].to_vec()));

        let (guid, position) = read_packed_guid(RefCell::clone(&reader));
        reader.borrow_mut().set_position(position);

        let (movement_info, _) = MovementParser::parse(RefCell::clone(&reader));

        input.data_storage.lock().unwrap().players_map.entry(guid).and_modify(|p| {
            p.position = Some(movement_info.position);
        });

        if input.session.lock().unwrap().me.as_ref().unwrap().guid != guid {
            let players_map = &mut input.data_storage.lock().unwrap().players_map;
            let player = players_map.get(&guid);

            if player.is_none() {
                let mut body = Vec::new();
                body.write_u64::<LittleEndian>(guid)?;

                let mut body = Vec::new();
                body.write_u64::<LittleEndian>(guid)?;

                return Ok(
                    HandlerOutput::Data(
                        OutcomePacket::from(Opcode::CMSG_NAME_QUERY, Some(body))
                    )
                );
            }
        }

        Ok(HandlerOutput::Void)
    }
}