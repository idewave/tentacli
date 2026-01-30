use std::fmt;
use std::io::Read;
use std::ops::Deref;
use binrw::{BinRead, BinResult};
use serde::{Deserialize, Serialize};

use crate::client::packet::{CalculateMetadata, MetadataContext, MetadataValue};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NullTerminated<T>(pub T);

impl fmt::Display for NullTerminated<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NullTerminated<String> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for NullTerminated<String> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BinRead for NullTerminated<String> {
    type Args<'a> = ();

    fn read_options<R: Read>(
        reader: &mut R,
        _: binrw::Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut buf = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            buf.push(byte[0]);
        }

        let s = String::from_utf8(buf)
            .map_err(|e| binrw::Error::Custom {
                pos: 0,
                err: Box::new(e),
            })?;

        Ok(NullTerminated(s))
    }
}

impl CalculateMetadata for NullTerminated<String> {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = self.0.len() + 1; // + '\0'
        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue {
                size,
                offset: ctx.offset,
            },
        );
        ctx.offset += size;
        ctx
    }
}

impl From<String> for NullTerminated<String> {
    fn from(value: String) -> Self {
        NullTerminated(value)
    }
}

impl From<&str> for NullTerminated<String> {
    fn from(value: &str) -> Self {
        NullTerminated(value.to_string())
    }
}