use std::fmt::Display;
use std::io::{BufRead, Write};
use async_trait::async_trait;
use byteorder::{WriteBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::io::{AsyncBufReadExt, AsyncBufRead};

use crate::primary::errors::FieldError;
use crate::primary::traits::BinaryConverter;
use crate::traits::StreamReader;

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

impl Display for TerminatedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(string) = self;
        write!(f, "{}", string)
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
            Err(err) => Err(FieldError::InvalidString(err, label.to_owned())),
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

#[async_trait]
impl StreamReader for TerminatedString {
    async fn read_from<R>(stream: &mut R) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        let mut internal_buf = vec![];
        let label = "TerminatedString";

        stream.read_until(0, &mut internal_buf).await
            .map_err(|e| FieldError::CannotRead(e, format!("bytes ({})", label)))?;
        match String::from_utf8(internal_buf[..internal_buf.len() - 1].to_vec()) {
            Ok(string) => Ok(Self(string)),
            Err(err) => Err(FieldError::InvalidString(err, label.to_owned())),
        }
    }
}