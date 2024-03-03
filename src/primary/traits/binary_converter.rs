use std::io::{BufRead, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::primary::client::{CooldownInfo, Player, Realm, Spell};
use crate::primary::errors::FieldError;
use crate::primary::parsers::movement_parser::MovementParser;
use crate::primary::parsers::movement_parser::types::MovementInfo;
use crate::primary::parsers::position_parser::types::Position;
use crate::primary::parsers::update_block_parser::{UpdateBlocksParser, types::ParsedBlock};

pub trait BinaryConverter {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError>;
    fn read_from<R: BufRead>(reader: R) -> Result<Self, FieldError> where Self: Sized;
}

impl BinaryConverter for u8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_u8(*self).map_err(|e| FieldError::CannotWrite(e, "u8".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_u8().map_err(|e| FieldError::CannotRead(e, "u8".to_string()))
    }
}

impl BinaryConverter for u16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_u16::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u16".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_u16::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u16".to_string()))
    }
}

impl BinaryConverter for u32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_u32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u32".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_u32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u32".to_string()))
    }
}

impl BinaryConverter for u64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_u64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u64".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_u64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u64".to_string()))
    }
}

impl BinaryConverter for i8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_i8(*self).map_err(|e| FieldError::CannotWrite(e, "i8".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_i8().map_err(|e| FieldError::CannotRead(e, "i8".to_string()))
    }
}

impl BinaryConverter for i16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_i16::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i16".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_i16::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i16".to_string()))
    }
}

impl BinaryConverter for i32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_i32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i32".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_i32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i32".to_string()))
    }
}

impl BinaryConverter for i64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_i64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i64".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_i64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i64".to_string()))
    }
}

impl BinaryConverter for f32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_f32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "f32".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_f32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "f32".to_string()))
    }
}

impl BinaryConverter for f64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_f64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "f64".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        reader.read_f64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "f64".to_string()))
    }
}

impl BinaryConverter for String {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_all(self.as_bytes()).map_err(|e| FieldError::CannotWrite(e, "String".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut internal_buf = vec![];
        reader.read_until(0, &mut internal_buf)
            .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
        String::from_utf8(
            internal_buf[..internal_buf.len()].to_vec()
        ).map_err(|e| FieldError::InvalidString(e, "String".to_string()))
    }
}

impl<const N: usize> BinaryConverter for [u8; N] {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_all(self).map_err(|e| FieldError::CannotWrite(e, "[u8; N]".to_string()))
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut internal_buf = [0; N];
        reader.read_exact(&mut internal_buf)
            .map_err(|e| FieldError::CannotRead(e, "[u8; N]".to_string()))?;
        Ok(internal_buf)
    }
}

impl BinaryConverter for Vec<ParsedBlock> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let parsed_blocks = UpdateBlocksParser::parse(&mut reader)
            .map_err(|e| FieldError::CannotRead(e, "Vec<ParsedBlock>".to_string()))?;
        Ok(parsed_blocks)
    }
}

impl BinaryConverter for Vec<Realm> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut realms = Vec::new();
        let label = "Vec<Realm>";

        let realms_count = reader.read_i16::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("realms_count:i16 ({})", label)))?;
        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("icon:u8 ({})", label)))?;
            let lock = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("lock:u8 ({})", label)))?;
            let flags = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("flags:u8 ({})", label)))?;

            reader.read_until(0, &mut name)
                .map_err(|e| FieldError::CannotRead(e, format!("name_buf:Vec<u8> ({})", label)))?;
            reader.read_until(0, &mut address)
                .map_err(|e| FieldError::CannotRead(e, format!("address_buf:Vec<u8> ({})", label)))?;

            let population = reader.read_f32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("population:f32 ({})", label)))?;
            let characters = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("characters:u8 ({})", label)))?;
            let timezone = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("timezone:u8 ({})", label)))?;
            let server_id = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("server_id:u8 ({})", label)))?;

            realms.push(Realm {
                icon,
                lock,
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

impl BinaryConverter for Vec<Player> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut characters = Vec::new();
        let label = "Vec<Character>";

        let characters_count = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("characters_count:u8 ({})", label)))?;
        for _ in 0 .. characters_count {
            let guid = reader.read_u64::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("guid:u64 ({})", label)))?;

            let mut name_buf = Vec::new();
            reader.read_until(0, &mut name_buf)
                .map_err(|e| FieldError::CannotRead(e, format!("name_buf:Vec<u8> ({})", label)))?;
            let name = String::from_utf8(
                name_buf[..(name_buf.len() - 1)].to_vec()
            ).map_err(|e| FieldError::InvalidString(e, label.to_owned()))?;

            let race = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("race:u8 ({})", label)))?;
            let class = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("class:u8 ({})", label)))?;
            let gender = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("gender:u8 ({})", label)))?;

            let _skin = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("skin:u8 ({})", label)))?;
            let _face = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("face:u8 ({})", label)))?;
            let _hair_style = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("hair_style:u8 ({})", label)))?;
            let _hair_color = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("hair_color:u8 ({})", label)))?;

            let _facial_hair = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("facial_hair:u8 ({})", label)))?;
            let level = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("level:u8 ({})", label)))?;

            let _zone_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("zone_id:u32 ({})", label)))?;
            let _map_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("map_id:u32 ({})", label)))?;

            let x = reader.read_f32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("x:f32 ({})", label)))?;
            let y = reader.read_f32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("y:f32 ({})", label)))?;
            let z = reader.read_f32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("z:f32 ({})", label)))?;

            let _guild_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("guild_id:u32 ({})", label)))?;
            let _char_flags = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("char_flags:u32 ({})", label)))?;
            let _char_customize_flags = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("char_customize_flags:u32 ({})", label)))?;

            let _first_login = reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("first_login:u8 ({})", label)))?;

            let _pet_display_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("pet_display_id:u32 ({})", label)))?;
            let _pet_level = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("pet_level:u32 ({})", label)))?;
            let _pet_family = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("pet_family:u32 ({})", label)))?;

            // inventory
            for _ in 0..23 {
                reader.read_u32::<LittleEndian>()
                    .map_err(|e| FieldError::CannotRead(e, format!("inventory:u32 ({})", label)))?;
                reader.read_u8()
                    .map_err(|e| FieldError::CannotRead(e, format!("inventory:u8 ({})", label)))?;
                reader.read_u32::<LittleEndian>()
                    .map_err(|e| FieldError::CannotRead(e, format!("inventory:u32_2 ({})", label)))?;
            }

            characters.push(Player {
                guid,
                name,
                race,
                class,
                gender,
                level,
                fields: Default::default(),
                movement_speed: Default::default(),
                position: Some(Position::new(x, y, z, 0.0)),
            });
        }

        Ok(characters)
    }
}

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> where Self: Sized {
        MovementParser::parse(&mut reader)
            .map_err(|e| FieldError::CannotRead(e, "MovementInfo".to_string()))
    }
}

impl BinaryConverter for Vec<Spell> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> where Self: Sized {
        let mut spells = Vec::new();
        let label = "Vec<Spell>";

        let spell_count = reader.read_u16::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("spell_count:u16 ({})", label)))?;
        for _ in 0..spell_count {
            let spell = Spell {
                spell_id: reader.read_u32::<LittleEndian>()
                    .map_err(|e| FieldError::CannotRead(e, format!("spell_id:u32 ({})", label)))?
            };
            reader.read_u16::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("unknown:u16 ({})", label)))?;

            spells.push(spell);
        }

        Ok(spells)
    }
}

impl BinaryConverter for Vec<CooldownInfo> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> where Self: Sized {
        let mut cooldowns = Vec::new();
        let label = "Vec<CooldownInfo>";

        let cooldown_count = reader.read_u16::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("unknown:u16 ({})", label)))?;
        for _ in 0..cooldown_count {
            let spell_id = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("spell_id:u32 ({})", label)))?;
            let item_id = reader.read_u16::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("item_id:u16 ({})", label)))?;
            let spell_category = reader.read_u16::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("spell_category:u16 ({})", label)))?;
            let cooldown_duration = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("cooldown_duration:u32 ({})", label)))?;
            let cooldown_category = reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("cooldown_category:u32 ({})", label)))?;

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
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_all(self).map_err(|e| FieldError::CannotWrite(e, "Vec<u8>".to_string()))
    }

    fn read_from<R: BufRead>(_reader: R) -> Result<Self, FieldError> where Self: Sized {
        todo!()
    }
}