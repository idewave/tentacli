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
    let mut characters: Vec<Character> = Vec::new();

    let mut reader = Cursor::new(input.data.as_ref().unwrap()[4..].to_vec());

    let characters_count = reader.read_u8()?;

    if characters_count == 0 {
        Ok(HandlerOutput::Void)
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

        let selected_character = characters.remove(0);
        let mut me = Player::new(
            selected_character.guid,
            selected_character.name,
            selected_character.race,
            selected_character.class
        );

        me.position = Some(selected_character.position);

        input.session.me = Some(me);

        Ok(HandlerOutput::Void)
    }
}