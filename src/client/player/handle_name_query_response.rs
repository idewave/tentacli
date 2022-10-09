use std::cell::RefCell;
use std::io::{BufRead, Cursor};
use byteorder::{ReadBytesExt};
use async_trait::async_trait;

use crate::client::Player;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;
use crate::utils::read_packed_guid;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[4..].to_vec()));
        let (guid, position) = read_packed_guid(RefCell::clone(&reader));
        reader.borrow_mut().set_position(position);

        reader.borrow_mut().read_u8()?;

        let mut name = Vec::new();
        reader.borrow_mut().read_until(0, &mut name)?;

        let name = match name.is_empty() {
            true => String::from("UNKNOWN"),
            false => String::from_utf8(name[..(name.len() - 1) as usize].to_owned()).unwrap(),
        };

        let mut realm = Vec::new();
        reader.borrow_mut().read_until(0, &mut realm)?;

        let race = reader.borrow_mut().read_u8()?;
        let _gender = reader.borrow_mut().read_u8()?;
        let class = reader.borrow_mut().read_u8()?;

        let my_guid = input.session.lock().unwrap().me.as_ref().unwrap().guid;

        let players_map = &mut input.data_storage.lock().unwrap().players_map;

        // modify/insert only another players
        // current player stored inside Session instance
        if my_guid != guid {
            players_map.entry(guid).and_modify(|p| {
                p.name = name.to_string();
                p.race = race;
                p.class = class;
            }).or_insert_with(|| Player::new(guid, name, race, class));
        }

        Ok(HandlerOutput::Void)
    }
}