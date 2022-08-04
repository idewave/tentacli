use std::io::{BufRead, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::client::movement::parsers::types::Position;
use crate::client::Player;

use super::types::Character;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session = input.session.lock().unwrap();
    if session.me.is_some() {
        return Ok(HandlerOutput::Void);
    }

    let mut characters: Vec<Character> = Vec::new();

    let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());

    let characters_count = reader.read_u8()?;

    if characters_count == 0 {
        return Ok(HandlerOutput::Void);
    } else {
        for _ in 0 .. characters_count {
            let guid = reader.read_u64::<LittleEndian>()?;
            let mut name = Vec::new();

            reader.read_until(0, &mut name)?;

            let race = reader.read_u8()?;
            let class = reader.read_u8()?;
            let gender = reader.read_u8()?;

            let _skin = reader.read_u8()?;
            let _face = reader.read_u8()?;
            let _hair_style = reader.read_u8()?;
            let _hair_color = reader.read_u8()?;

            let _facial_hair = reader.read_u8()?;
            let level = reader.read_u8()?;

            let _zone_id = reader.read_u32::<LittleEndian>()?;
            let _map_id = reader.read_u32::<LittleEndian>()?;

            let x = reader.read_f32::<LittleEndian>()?;
            let y = reader.read_f32::<LittleEndian>()?;
            let z = reader.read_f32::<LittleEndian>()?;

            let _guild_id = reader.read_u32::<LittleEndian>()?;
            let _char_flags = reader.read_u32::<LittleEndian>()?;
            let _char_customize_flags = reader.read_u32::<LittleEndian>()?;

            let _first_login = reader.read_u8()?;

            let _pet_display_id = reader.read_u32::<LittleEndian>()?;
            let _pet_level = reader.read_u32::<LittleEndian>()?;
            let _pet_family = reader.read_u32::<LittleEndian>()?;

            // inventory
            for _ in 0..23 {
                reader.read_u32::<LittleEndian>()?;
                reader.read_u8()?;
                reader.read_u32::<LittleEndian>()?;
            }

            characters.push(Character {
                guid,
                name: String::from_utf8(name[..(name.len() - 1) as usize].to_owned()).unwrap(),
                race,
                class,
                gender,
                level,
                position: Position::new(x, y, z, 0.0),
            });
        }

        input.dialog_income.send_characters_dialog(characters);

        Ok(HandlerOutput::Freeze)
    }
}