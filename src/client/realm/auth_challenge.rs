use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Write, Read};
use sha1::{Digest, Sha1};
use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::random_range;

const CLIENT_SEED_SIZE: usize = 4;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session = input.session.lock().unwrap();
    let server_id = session.selected_realm.as_ref().unwrap().server_id;
    let config = session.get_config().unwrap();
    let username = &config.connection_data.username;
    let session_key = session.session_key.as_ref().unwrap();

    let mut reader = Cursor::new(input.data.as_ref().unwrap()[8..].to_vec());
    let mut server_seed = vec![0u8; 32];
    reader.read_exact(&mut server_seed)?;

    let client_seed = random_range(CLIENT_SEED_SIZE);

    let hasher = Sha1::new();
    let digest = hasher
        .chain(&username)
        .chain(vec![0, 0, 0, 0])
        .chain(&client_seed)
        // from server_seed we need only CLIENT_SEED_SIZE first bytes
        .chain(&server_seed[..CLIENT_SEED_SIZE])
        .chain(session_key)
        .finalize();

    let mut body = Vec::new();
    // TODO: refactor build into config or smth like that
    body.write_i32::<LittleEndian>(12340)?;
    body.write_u32::<LittleEndian>(0)?;
    body.write_all(username.as_bytes())?;
    body.write_u8(0)?;
    body.write_u32::<LittleEndian>(0)?;
    body.write_all(&client_seed)?;
    body.write_u32::<LittleEndian>(0)?;
    body.write_u32::<LittleEndian>(0)?;
    body.write_u32::<LittleEndian>(server_id as u32)?;
    body.write_u32::<LittleEndian>(2)?; // expansion ???
    body.write_u32::<LittleEndian>(0)?; // ???
    body.write_all(&digest)?;

    let mut addon_info = Vec::new();
    addon_info.write_u32::<LittleEndian>(config.addons.len() as u32)?;

    for addon in &config.addons {
        addon_info.write_all(addon.name.as_bytes())?;
        addon_info.write_u8(0)?; // null-terminator for name string
        addon_info.write_u8(addon.flags)?;
        addon_info.write_u32::<LittleEndian>(addon.modulus_crc)?;
        addon_info.write_u32::<LittleEndian>(addon.urlcrc_crc)?;
    }

    // seems like this timestamp always same, maybe it can be moved to config or smth ?
    addon_info.write_u32::<LittleEndian>(1636457673)?; // last modified timestamp, smth like that

    body.write_u32::<LittleEndian>(addon_info.len() as u32)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&addon_info)?;
    body.write_all(&encoder.finish().unwrap())?;

    input.message_income.send_client_message(String::from("CMSG_AUTH_SESSION"));

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_AUTH_SESSION, Some(body))))
}