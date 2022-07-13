use std::cell::RefCell;
use std::io::{Cursor};

use crate::client::movement::parsers::movement_parser::MovementParser;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::{read_packed_guid};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[4..].to_vec()));

    let (guid, position) = read_packed_guid(RefCell::clone(&reader));
    reader.borrow_mut().set_position(position);

    if input.session.me.as_ref().unwrap().guid == guid {
        println!("CURRENT PLAYER MOVE!");
    }

    let (movement_info, _) = MovementParser::parse(RefCell::clone(&reader));

    input.data_storage.players_map.entry(guid).and_modify(|p| {
        p.position = Some(movement_info.position);
    });

    // println!("TARGET POS: {:?}", movement_info.position);
    // println!("BOT POS: {:?}", input.session.me.as_ref().unwrap().position);

    Ok(HandlerOutput::Void)
}