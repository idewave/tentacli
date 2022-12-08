use std::io::{BufRead, Error, ErrorKind, Write};
use byteorder::ReadBytesExt;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeTupleStruct};

use crate::traits::binary_converter::BinaryConverter;

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

impl<'de> Deserialize<'de> for PackedGuid {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for PackedGuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 1;
        let mut state = serializer.serialize_tuple_struct("PackedGuid", FIELDS_AMOUNT)?;
        state.serialize_field(&self.0)?;
        state.end()
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