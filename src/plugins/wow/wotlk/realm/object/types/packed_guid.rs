use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::Serialize;

use crate::client::prelude::*;

#[derive(Serialize, Default, PartialEq, Clone, Copy, Eq, Hash, Debug)]
pub struct PackedGuid(pub u64);

impl CalculateMetadata for PackedGuid {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let guid = self.0;

        let mut size = 1usize;

        for i in 0..8 {
            if ((guid >> (i * 8)) & 0xFF) != 0 {
                size += 1;
            }
        }

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

impl PackedGuid {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl PartialEq<u64> for PackedGuid {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<PackedGuid> for u64 {
    fn eq(&self, other: &PackedGuid) -> bool {
        *self == other.0
    }
}

impl BinRead for PackedGuid {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mask = <u8>::read_options(reader, endian, args)?;

        if mask == 0 {
            return Ok(Self(0));
        }

        let mut guid: u64 = 0;
        let mut i = 0;

        while i < 8 {
            if (mask & (1 << i)) != 0 {
                guid |= (<u8>::read_options(reader, endian, args)? as u64) << (i * 8);
            }
            i += 1;
        }

        Ok(Self(guid))
    }
}

impl BinWrite for PackedGuid {
    type Args<'a> = ();

    fn write<W: Write>(&self, writer: &mut W) -> BinResult<()> {
        let mut guid = self.0;
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

        writer.write_all(&packed_guid[..size])?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(
        &self, writer: &mut W,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        let mut guid = self.0;
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

        writer.write_all(&packed_guid[..size])?;
        Ok(())
    }
}