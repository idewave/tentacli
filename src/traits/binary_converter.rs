use std::io::{BufRead, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::client::{Character, CooldownInfo, Realm, Spell};
use crate::errors::IOError;
use crate::parsers::movement_parser::MovementParser;
use crate::parsers::movement_parser::types::MovementInfo;
use crate::parsers::position_parser::types::Position;
use crate::parsers::update_block_parser::{UpdateBlocksParser, types::ParsedBlock};

pub trait BinaryConverter {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError>;
    fn read_from<R: BufRead>(reader: R) -> Result<Self, IOError> where Self: Sized;
}

impl BinaryConverter for u8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_u8(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_u8().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for u16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_u16::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_u16::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for u32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_u32::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_u32::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for u64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_u64::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_u64::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for i8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_i8(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_i8().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for i16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_i16::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_i16::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for i32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_i32::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_i32::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for i64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_i64::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_i64::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for f32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_f32::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_f32::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for f64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_f64::<LittleEndian>(*self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        reader.read_f64::<LittleEndian>().map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for String {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_all(self.as_bytes()).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        let mut internal_buf = vec![];
        reader.read_until(0, &mut internal_buf)
            .map_err(|e| IOError::ReadError(e))?;
        String::from_utf8(
            internal_buf[..internal_buf.len()].to_vec()
        ).map_err(|e| IOError::StringReadError(e))
    }
}

impl<const N: usize> BinaryConverter for [u8; N] {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_all(self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        let mut internal_buf = [0; N];
        reader.read_exact(&mut internal_buf).map_err(|e| IOError::ReadError(e))?;
        Ok(internal_buf)
    }
}

impl BinaryConverter for Vec<ParsedBlock> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        let parsed_blocks = UpdateBlocksParser::parse(&mut reader)
            .map_err(|e| IOError::ReadError(e))?;
        Ok(parsed_blocks)
    }
}

impl BinaryConverter for Vec<Realm> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        let mut realms = Vec::new();

        let realms_count = reader.read_i16::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;
        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = reader.read_u16::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let flags = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            reader.read_until(0, &mut name)
                .map_err(|e| IOError::ReadError(e))?;
            reader.read_until(0, &mut address)
                .map_err(|e| IOError::ReadError(e))?;

            let population = reader.read_f32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let characters = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let timezone = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let server_id = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            realms.push(Realm {
                icon,
                flags,
                name: String::from_utf8_lossy(&name).trim_matches(char::from(0)).to_string(),
                address: String::from_utf8_lossy(&address).trim_matches(char::from(0)).to_string(),
                population,
                characters,
                timezone,
                server_id,
            });
        }

        Ok(realms)
    }
}

impl BinaryConverter for Vec<Character> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> {
        let mut characters = Vec::new();

        let characters_count = reader.read_u8().map_err(|e| IOError::ReadError(e))?;
        for _ in 0 .. characters_count {
            let guid = reader.read_u64::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            let mut name_buf = Vec::new();
            reader.read_until(0, &mut name_buf)
                .map_err(|e| IOError::ReadError(e))?;
            let name = String::from_utf8(
                name_buf[..(name_buf.len() - 1) as usize].to_vec()
            ).map_err(|e| IOError::StringReadError(e))?;

            let race = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let class = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let gender = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            let _skin = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let _face = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let _hair_style = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let _hair_color = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            let _facial_hair = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;
            let level = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            let _zone_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let _map_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            let x = reader.read_f32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let y = reader.read_f32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let z = reader.read_f32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            let _guild_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let _char_flags = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let _char_customize_flags = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            let _first_login = reader.read_u8()
                .map_err(|e| IOError::ReadError(e))?;

            let _pet_display_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let _pet_level = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let _pet_family = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            // inventory
            for _ in 0..23 {
                reader.read_u32::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;
                reader.read_u8().map_err(|e| IOError::ReadError(e))?;
                reader.read_u32::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;
            }

            characters.push(Character {
                guid,
                name,
                race,
                class,
                gender,
                level,
                position: Position::new(x, y, z, 0.0),
            });
        }

        Ok(characters)
    }
}

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> where Self: Sized {
        MovementParser::parse(&mut reader).map_err(|e| IOError::ReadError(e))
    }
}

impl BinaryConverter for Vec<Spell> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> where Self: Sized {
        let mut spells = Vec::new();

        let spell_count = reader.read_u16::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;
        for _ in 0..spell_count {
            let spell = Spell {
                spell_id: reader.read_u32::<LittleEndian>().map_err(|e| IOError::ReadError(e))?
            };
            reader.read_u16::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;

            spells.push(spell);
        }

        Ok(spells)
    }
}

impl BinaryConverter for Vec<CooldownInfo> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), IOError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, IOError> where Self: Sized {
        let mut cooldowns = Vec::new();

        let cooldown_count = reader.read_u16::<LittleEndian>().map_err(|e| IOError::ReadError(e))?;
        for _ in 0..cooldown_count {
            let spell_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let item_id = reader.read_u16::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let spell_category = reader.read_u16::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let cooldown_duration = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;
            let cooldown_category = reader.read_u32::<LittleEndian>()
                .map_err(|e| IOError::ReadError(e))?;

            cooldowns.push(CooldownInfo {
                spell_id,
                item_id,
                spell_category,
                cooldown_duration,
                cooldown_category,
            })
        }

        Ok(cooldowns)
    }
}

impl BinaryConverter for Vec<u8> {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), IOError> {
        buffer.write_all(self).map_err(|e| IOError::WriteError(e))
    }

    fn read_from<R: BufRead>(_reader: R) -> Result<Self, IOError> where Self: Sized {
        todo!()
    }
}