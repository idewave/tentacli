use anyhow::{Result as AnyResult};
use std::io::{BufRead, Write};
use byteorder::ReadBytesExt;
use serde::{Serialize, Serializer};

use crate::{BinaryConverter, FieldError};

#[derive(Debug, Default, Clone)]
pub struct PackedGuid(pub u64);

impl PartialEq<u64> for PackedGuid {
    fn eq(&self, other: &u64) -> bool {
        let PackedGuid(guid) = self;
        guid == other
    }
}

impl PartialEq<PackedGuid> for u64 {
    fn eq(&self, other: &PackedGuid) -> bool {
        let PackedGuid(guid) = other;
        guid == self
    }
}

impl Serialize for PackedGuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_u64(self.0)
    }
}

impl BinaryConverter for PackedGuid {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        let PackedGuid(mut guid) = self;
        let mut packed_guid = [0u8; 9];
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

        buffer.write_all(&packed_guid[..size])
            .map_err(|e| FieldError::CannotWrite(e, "bytes (PackedGuid)".to_string()))?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let mask = reader.read_u8().unwrap_or(0);

        if mask == 0 {
            return Ok(PackedGuid(0));
        }

        let mut guid: u64 = 0;
        let mut i = 0;

        while i < 8 {
            if (mask & (1 << i)) != 0 {
                guid |= (reader.read_u8()
                    .map_err(|e| FieldError::CannotRead(
                        e, "guid:u8 of PackedGuid".to_string())
                    )? as u64) << (i * 8);
            }

            i += 1;
        }

        Ok(PackedGuid(guid))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let PackedGuid(guid) = self;
        guid.to_le_bytes().to_vec()
    }
}