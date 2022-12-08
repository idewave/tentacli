use std::io::{BufRead, Error, ErrorKind, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeTupleStruct};

use crate::traits::binary_converter::BinaryConverter;

#[derive(Debug, Default, Clone)]
pub struct TerminatedString(String);

impl From<&str> for TerminatedString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for TerminatedString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl BinaryConverter for TerminatedString {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        let TerminatedString(str) = self;
        buffer.write_all(str.as_bytes())?;
        buffer.write_u8(0)?;
        Ok(())
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut internal_buf = vec![];
        reader.read_until(0, &mut internal_buf)?;
        match String::from_utf8(internal_buf[..internal_buf.len()].to_vec()) {
            Ok(string) => Ok(Self(string)),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for TerminatedString {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for TerminatedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 1;
        let mut state = serializer.serialize_tuple_struct("TerminatedString", FIELDS_AMOUNT)?;
        state.serialize_field(&self.0)?;
        state.end()
    }
}