use std::cell::RefCell;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::client::movement::parsers::types::Position;

pub struct PositionParser;

impl PositionParser {
    pub fn parse(reader: RefCell<Cursor<Vec<u8>>>) -> (Position, u64) {
        let x = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
        let y = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
        let z = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
        let orientation = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();

        (Position::new(x, y, z, orientation), reader.borrow_mut().position())
    }
}