use std::io::{BufRead, Cursor, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::types::errors::FieldError;

pub trait BinaryConverter {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()>;
    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl BinaryConverter for bool {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u8(*self as u8)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_u8()? == 1)
    }
}

impl BinaryConverter for u8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u8(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl BinaryConverter for u16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u16::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl BinaryConverter for u32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u32::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl BinaryConverter for u64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u64::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl BinaryConverter for i8 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_i8(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_i8()?)
    }
}

impl BinaryConverter for i16 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_i16::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_i16::<LittleEndian>()?)
    }
}

impl BinaryConverter for i32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_i32::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl BinaryConverter for i64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_i64::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl BinaryConverter for f32 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_f32::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_f32::<LittleEndian>()?)
    }
}

impl BinaryConverter for f64 {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_f64::<LittleEndian>(*self)?;
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        Ok(reader.read_f64::<LittleEndian>()?)
    }
}

impl BinaryConverter for String {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_all(self.as_bytes())?;

        Ok(())
    }

    fn read_from<R: BufRead>(
        reader: &mut R,
        dependencies: &mut Vec<u8>,
    ) -> anyhow::Result<Self> {
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
            reader.read_until(0, &mut buffer)?;
            buffer
        };

        let string = String::from_utf8(buffer)?;

        Ok(string.trim_end_matches(char::from(0)).to_string())
    }
}

impl<const N: usize, T> BinaryConverter for [T; N]
where
    T: Sized + BinaryConverter + Clone + core::fmt::Debug,
{
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        for i in 0..self.len() {
            self[i].write_into(buffer)?;
        }
        
        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        let mut buffer = Vec::with_capacity(N);
        for _ in 0..N {
            buffer.push(T::read_from(reader, &mut vec![])?);
        }

        Ok(buffer.try_into().unwrap())
    }
}

impl<T: BinaryConverter + Clone> BinaryConverter for Vec<T>
where
    T: Sized,
{
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        self.iter_mut().for_each(|item| item.write_into(buffer).unwrap());

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(dependencies.to_vec());
        let size = match dependencies.len() {
            1 => ReadBytesExt::read_u8(&mut cursor)? as usize,
            2 => ReadBytesExt::read_u16::<LittleEndian>(&mut cursor)? as usize,
            _ => ReadBytesExt::read_u32::<LittleEndian>(&mut cursor)? as usize,
        };
        let mut buffer: Vec<T> = vec![];

        for _ in 0..size {
            buffer.push(T::read_from(reader, dependencies)?);
        }

        Ok(buffer)
    }
}