use anyhow::{Result as AnyResult};
use std::io::{BufRead, Cursor, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::errors::FieldError;

pub trait BinaryConverter {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()>;
    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self>
        where Self: Sized;

    // fn read_from_with_logging<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self>
    // where Self: Sized + std::fmt::Debug
    // {
    //     println!("Calling read_from for type: {}", std::any::type_name::<Self>());
    //     let result = Self::read_from(reader, dependencies);
    //
    //     match &result {
    //         Ok(value) => println!("Successfully read value: {:?}", value),
    //         Err(e) => println!("Failed to read value: {:?}", e),
    //     }
    //
    //     result
    // }
}

impl BinaryConverter for u8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_u8(*self).map_err(|e| FieldError::CannotWrite(e, "u8".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_u8().map_err(|e| FieldError::CannotRead(e, "u8".to_string()).into())
    }
}

impl BinaryConverter for u16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_u16::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u16".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_u16::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u16".to_string()).into())
    }
}

impl BinaryConverter for u32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_u32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u32".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_u32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u32".to_string()).into())
    }
}

impl BinaryConverter for u64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_u64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "u64".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_u64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "u64".to_string()).into())
    }
}

impl BinaryConverter for i8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_i8(*self).map_err(|e| FieldError::CannotWrite(e, "i8".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_i8().map_err(|e| FieldError::CannotRead(e, "i8".to_string()).into())
    }
}

impl BinaryConverter for i16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_i16::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i16".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_i16::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i16".to_string()).into())
    }
}

impl BinaryConverter for i32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_i32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i32".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_i32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i32".to_string()).into())
    }
}

impl BinaryConverter for i64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_i64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "i64".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_i64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "i64".to_string()).into())
    }
}

impl BinaryConverter for f32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_f32::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "f32".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_f32::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "f32".to_string()).into())
    }
}

impl BinaryConverter for f64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_f64::<LittleEndian>(*self)
            .map_err(|e| FieldError::CannotWrite(e, "f64".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        reader.read_f64::<LittleEndian>().map_err(|e| FieldError::CannotRead(e, "f64".to_string()).into())
    }
}

impl BinaryConverter for String {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_all(self.as_bytes())
            .map_err(|e| FieldError::CannotWrite(e, "String".to_string()))?;

        Ok(())
    }

    fn read_from<R: BufRead>(
        reader: &mut R,
        dependencies: &mut Vec<u8>
    ) -> AnyResult<Self> {
        let mut cursor = Cursor::new(dependencies.to_vec());

        let size = match dependencies.len() {
            1 => ReadBytesExt::read_u8(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "String u8 size".to_string()))? as usize,
            2 => ReadBytesExt::read_u16::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "String u16 size".to_string()))? as usize,
            4 => ReadBytesExt::read_u32::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "String u32 size".to_string()))? as usize,
            _ => 0,
        };

        let buffer = if size > 0 {
            let mut buffer = vec![0u8; size];
            reader.read_exact(&mut buffer)
                .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
            buffer
        } else {
            let mut buffer = vec![];
            reader.read_until(0, &mut buffer)
                .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
            buffer
        };

        let string = String::from_utf8(buffer)
            .map_err(|e| FieldError::InvalidString(e, "String".to_string()))?;

        Ok(string.trim_end_matches(char::from(0)).to_string())
    }
}

impl<const N: usize> BinaryConverter for [u8; N] {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        buffer.write_all(self).map_err(|e| FieldError::CannotWrite(e, "[u8; N]".to_string()).into())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let mut internal_buf = [0; N];
        reader.read_exact(&mut internal_buf)
            .map_err(|e| FieldError::CannotRead(e, "[u8; N]".to_string()))?;
        Ok(internal_buf)
    }
}

impl<T: BinaryConverter + Clone> BinaryConverter for Vec<T> where T: Sized {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.iter_mut().for_each(|item| item.write_into(buffer).unwrap());

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self> {
        let mut cursor = Cursor::new(dependencies.to_vec());
        let size = match dependencies.len() {
            1 => ReadBytesExt::read_u8(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "Vec<T> u8 size".to_string()))? as usize,
            2 => ReadBytesExt::read_u16::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "Vec<T> u16 size".to_string()))? as usize,
            _ => ReadBytesExt::read_u32::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, "Vec<T> u32 size".to_string()))? as usize,
        };
        let mut buffer: Vec<T> = vec![];

        for _ in 0..size {
            buffer.push(T::read_from(reader, dependencies)?);
        }

        Ok(buffer)
    }
}