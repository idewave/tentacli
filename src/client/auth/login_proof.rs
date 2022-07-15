use std::io::{Cursor, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};
use sha1::{Sha1};

use super::opcodes::Opcode;

use crate::crypto::srp::Srp;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::random_range;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let config = &input.session.get_config();

    let mut reader = Cursor::new(input.data.as_ref().unwrap()[3..].to_vec());

    let mut server_ephemeral = vec![0u8; 32];
    reader.read_exact(&mut server_ephemeral)?;
    let g_len = reader.read_u8()?;
    let mut g: Vec<u8> = vec![0u8; g_len as usize];
    reader.read_exact(&mut g)?;
    let n_len = reader.read_u8()?;
    let mut n: Vec<u8> = vec![0u8; n_len as usize];
    reader.read_exact(&mut n)?;
    let mut salt = vec![0u8; 32];
    reader.read_exact(&mut salt)?;

    let username = &config.connection_data.username;
    let password = &config.connection_data.password;

    let mut srp_client = Srp::new(n, g, server_ephemeral);
    let proof = srp_client.calculate_proof::<Sha1>(username, password, &salt);

    let crc_hash = random_range(20);

    let mut header = Vec::new();
    header.write_u8(Opcode::LOGIN_PROOF)?;

    let mut body = Vec::new();
    body.write_all(&srp_client.public_ephemeral())?;
    body.write_all(&proof)?;
    body.write_all(&crc_hash)?;
    body.write_u8(0)?; // number of keys
    body.write_u8(0)?; // security flags

    input.session.session_key = Some(srp_client.session_key());

    Ok(HandlerOutput::Data((Opcode::LOGIN_PROOF as u32, header, body)))
}