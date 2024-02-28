use std::io::{Error, Write};
use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Clone, Debug)]
pub struct ConnectionData {
    pub account: String,
    pub password: String,
    pub autoselect_realm_name: String,
    pub autoselect_character_name: String,
}

#[derive(Clone, Debug)]
pub struct AddonInfo {
    pub name: String,
    pub flags: u8,
    pub modulus_crc: u32,
    pub urlcrc_crc: u32,
}

impl AddonInfo {
    pub fn build_addon_info(addons: Vec<AddonInfo>) -> Result<Vec<u8>, Error> {
        let mut addon_info = Vec::new();
        addon_info.write_u32::<LittleEndian>(addons.len() as u32)?;

        for addon in addons {
            addon_info.write_all(addon.name.as_bytes())?;
            addon_info.write_u8(0)?; // null-terminator for name string
            addon_info.write_u8(addon.flags)?;
            addon_info.write_u32::<LittleEndian>(addon.modulus_crc)?;
            addon_info.write_u32::<LittleEndian>(addon.urlcrc_crc)?;
        }

        // seems like this timestamp always same, maybe it can be moved to config or smth ?
        // last modified timestamp, smth like that
        addon_info.write_u32::<LittleEndian>(1636457673)?;

        Ok(addon_info)
    }
}

#[derive(Clone, Debug)]
pub struct ChannelLabels {
    pub lfg: String,
    pub common: String,
    pub trade: String,
}