use std::io::{BufRead, Cursor, Error, ErrorKind};
use std::str::FromStr;
use byteorder::{LittleEndian, ReadBytesExt};
use regex::Regex;

use crate::client::realm::types::Realm;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let config = &input.session.get_config();
    let realm_name_pattern = &config.connection_data.realm_name;
    let re = Regex::new(realm_name_pattern.as_str()).unwrap();

    let mut realms: Vec<Realm> = Vec::new();

    // omit opcode and first 6 bytes (unknown)
    let mut reader = Cursor::new(input.data.as_ref().unwrap()[7..].to_vec());

    let realms_count = reader.read_i16::<LittleEndian>().unwrap();

    for _ in 0 .. realms_count {
        let mut name = Vec::new();
        let mut address = Vec::new();

        let icon = reader.read_u16::<LittleEndian>().unwrap();
        let flags = reader.read_u8().unwrap();

        reader.read_until(0, &mut name).unwrap();
        reader.read_until(0, &mut address).unwrap();

        let population = reader.read_f32::<LittleEndian>().unwrap();
        let characters = reader.read_u8().unwrap();
        let timezone = reader.read_u8().unwrap();
        let server_id = reader.read_u8().unwrap();

        input.session.server_id = Some(server_id);

        realms.push(Realm {
            icon,
            flags,
            name: String::from_utf8(name[..(name.len() - 1) as usize].to_owned()).unwrap(),
            address: String::from_utf8(address[..(address.len() - 1) as usize].to_owned()).unwrap(),
            population,
            characters,
            timezone,
            server_id,
        });
    }

    println!("REALMS: {:?}", &realms);

    let get_realm = || realms.into_iter().find(|item| re.is_match(&item.name[..]));

    return match get_realm() {
        Some(realm) => {
            // https://rust-lang.github.io/rust-clippy/master/index.html#single_char_pattern
            let connection_data: Vec<&str> = realm.address.split(':').collect();

            println!("REALM: {:?}", &realm);

            let host = connection_data[0].to_string();
            let port = u16::from_str(connection_data[1]).unwrap();

            Ok(HandlerOutput::ConnectionRequest(host, port))
        }
        _ => {
            Err(Error::new(ErrorKind::Other, "No realm found !"))
        }
    }
}