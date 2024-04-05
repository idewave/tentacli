use std::io::{BufRead, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::primary::errors::FieldError;

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

impl BinaryConverter for Vec<u8> {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        buffer.write_all(self).map_err(|e| FieldError::CannotWrite(e, "Vec<u8>".to_string()))
    }

    fn read_from<R: BufRead>(_reader: R) -> Result<Self, FieldError> where Self: Sized {
        todo!()
    }
}