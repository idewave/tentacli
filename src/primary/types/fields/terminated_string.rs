use std::io::{BufRead, Write};
use byteorder::{WriteBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::primary::errors::FieldError;
use crate::primary::traits::binary_converter::BinaryConverter;

#[derive(Debug, Default, Clone)]
pub struct TerminatedString(pub String);

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

impl From<Vec<u8>> for TerminatedString {
    fn from(value: Vec<u8>) -> Self {
        let string = String::from_utf8(value).unwrap();
        Self(string)
    }
}

impl ToString for TerminatedString {
    fn to_string(&self) -> String {
        let Self(string) = self;
        string.to_string()
    }
}

impl BinaryConverter for TerminatedString {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        let TerminatedString(str) = self;
        let label = "TerminatedString";

        buffer.write_all(str.as_bytes())
            .map_err(|e| FieldError::CannotWrite(e, format!("bytes ({})", label)))?;
        buffer.write_u8(0)
            .map_err(|e| FieldError::CannotWrite(e, format!("u8 zero ({})", label)))?;
        Ok(())
    }

    fn read_from<R: BufRead>(mut reader: R) -> Result<Self, FieldError> {
        let mut internal_buf = vec![];
        let label = "TerminatedString";

        reader.read_until(0, &mut internal_buf)
            .map_err(|e| FieldError::CannotRead(e, format!("bytes ({})", label)))?;
        match String::from_utf8(internal_buf[..internal_buf.len() - 1].to_vec()) {
            Ok(string) => Ok(Self(string)),
            Err(err) => Err(FieldError::InvalidString(err, format!("{}", label))),
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
        serializer.serialize_str(self.0.trim_end_matches(char::from(0)))
    }
}