use std::io::{BufRead, Error};
use byteorder::{LittleEndian, ReadBytesExt};

pub mod types;

use crate::parsers::position_parser::types::Position;
pub struct PositionParser;

impl PositionParser {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<Position, Error> {
        let x = reader.read_f32::<LittleEndian>()?;
        let y = reader.read_f32::<LittleEndian>()?;
        let z = reader.read_f32::<LittleEndian>()?;
        let orientation = reader.read_f32::<LittleEndian>()?;

        Ok(Position::new(x, y, z, orientation))
    }
}