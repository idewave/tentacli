use std::io::{BufRead, Error, ErrorKind, Write};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::client::{Character, CooldownInfo, Realm, Spell};
use crate::parsers::movement_parser::MovementParser;
use crate::parsers::movement_parser::types::MovementInfo;
use crate::parsers::position_parser::types::Position;
use crate::parsers::update_block_parser::{UpdateBlocksParser, types::ParsedBlock};
use crate::types::{PackedGuid, TerminatedString};

pub trait BinaryConverter {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error>;
    fn read_from<R: BufRead>(reader: R) -> Result<Self, Error> where Self: Sized;
}

impl BinaryConverter for u8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_u8(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_u8()
    }
}

impl BinaryConverter for u16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_u16::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_u16::<LittleEndian>()
    }
}

impl BinaryConverter for u32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_u32::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_u32::<LittleEndian>()
    }
}

impl BinaryConverter for u64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_u64::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_u64::<LittleEndian>()
    }
}

impl BinaryConverter for i8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_i8(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_i8()
    }
}

impl BinaryConverter for i16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_i16::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_i16::<LittleEndian>()
    }
}

impl BinaryConverter for i32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_i32::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_i32::<LittleEndian>()
    }
}

impl BinaryConverter for i64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_i64::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_i64::<LittleEndian>()
    }
}

impl BinaryConverter for f32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_f32::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_f32::<LittleEndian>()
    }
}

impl BinaryConverter for f64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_f64::<LittleEndian>(*self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        reader.read_f64::<LittleEndian>()
    }
}

impl BinaryConverter for String {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_all(self.as_bytes())
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut internal_buf = vec![];
        reader.read_until(0, &mut internal_buf)?;
        match String::from_utf8(internal_buf[..internal_buf.len()].to_vec()) {
            Ok(string) => Ok(string),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}

impl BinaryConverter for TerminatedString {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_all(format!("{}\0", self.get_value()).as_bytes())
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut internal_buf = vec![];
        reader.read_until(0, &mut internal_buf)?;
        match String::from_utf8(internal_buf[..internal_buf.len()].to_vec()) {
            Ok(string) => Ok(TerminatedString::from(string)),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}

impl<const N: usize> BinaryConverter for [u8; N] {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_all(self)
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut internal_buf = [0; N];
        reader.read_exact(&mut internal_buf)?;
        Ok(internal_buf)
    }
}

impl BinaryConverter for Vec<ParsedBlock> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let parsed_blocks = UpdateBlocksParser::parse(&mut reader)?;
        Ok(parsed_blocks)
    }
}

impl BinaryConverter for Vec<Realm> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut realms = Vec::new();

        let realms_count = reader.read_i16::<LittleEndian>()?;
        for _ in 0 .. realms_count {
            let mut name = Vec::new();
            let mut address = Vec::new();

            let icon = reader.read_u16::<LittleEndian>()?;
            let flags = reader.read_u8()?;

            reader.read_until(0, &mut name)?;
            reader.read_until(0, &mut address)?;

            let population = reader.read_f32::<LittleEndian>()?;
            let characters = reader.read_u8()?;
            let timezone = reader.read_u8()?;
            let server_id = reader.read_u8()?;

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
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut characters = Vec::new();

        let characters_count = reader.read_u8()?;
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

        Ok(characters)
    }
}

impl BinaryConverter for PackedGuid {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        let PackedGuid(mut guid) = self;
        let mut packed_guid = vec![0u8; 9];
        let mut size = 1;
        let mut index = 0;

        while guid != 0 {
            if guid & 0xFF > 0 {
                packed_guid[0] |= 1 << index;
                packed_guid[size] = guid as u8;
                size += 1;
            }

            index += 1;
            guid >>= 8;
        }

        buffer.write_all(&packed_guid[..size].to_vec())?;

        Ok(())
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mask = reader.read_u8().unwrap_or(0);

        if mask == 0 {
            return Err(Error::new(ErrorKind::Other, "Cannot read from"));
        }

        let mut guid: u64 = 0;
        let mut i = 0;

        while i < 8 {
            if (mask & (1 << i)) != 0 {
                guid |= (reader.read_u8().unwrap() as u64) << (i * 8);
            }

            i += 1;
        }

        Ok(PackedGuid(guid))
    }
}

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> where Self: Sized {
        MovementParser::parse(&mut reader)
    }
}

impl BinaryConverter for Vec<Spell> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> where Self: Sized {
        let mut spells = Vec::new();

        let spell_count = reader.read_u16::<LittleEndian>()?;
        for _ in 0..spell_count {
            let spell = Spell {
                spell_id: reader.read_u32::<LittleEndian>()?
            };
            reader.read_u16::<LittleEndian>()?;

            spells.push(spell);
        }

        Ok(spells)
    }
}

impl BinaryConverter for Vec<CooldownInfo> {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), Error> {
        todo!()
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> where Self: Sized {
        let mut cooldowns = Vec::new();

        let cooldown_count = reader.read_u16::<LittleEndian>()?;
        for _ in 0..cooldown_count {
            let spell_id = reader.read_u32::<LittleEndian>()?;
            let item_id = reader.read_u16::<LittleEndian>()?;
            let spell_category = reader.read_u16::<LittleEndian>()?;
            let cooldown_duration = reader.read_u32::<LittleEndian>()?;
            let cooldown_category = reader.read_u32::<LittleEndian>()?;

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
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.write_all(self)
    }

    fn read_from<R: BufRead>(_reader: R) -> Result<Self, Error> where Self: Sized {
        todo!()
    }
}

impl BinaryConverter for crate::types::IpAddr {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        let crate::types::IpAddr(value) = self;
        buffer.write_u32::<BigEndian>(*value)
    }

    fn read_from<R: BufRead>(_reader: R) -> Result<Self, Error> {
        todo!()
    }
}