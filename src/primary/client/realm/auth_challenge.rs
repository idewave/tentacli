use std::io::{Write};
use sha1::{Digest, Sha1};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use async_trait::async_trait;

use crate::primary::macros::with_opcode;
use crate::primary::client::opcodes::Opcode;
use crate::primary::config::types::AddonInfo;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::primary::traits::packet_handler::PacketHandler;

const CLIENT_SEED_SIZE: usize = 4;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    skip: u32,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    server_seed: [u8; 32],
}

with_opcode! {
    @world_opcode(Opcode::CMSG_AUTH_SESSION)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        build: u32,
        unknown: u32,
        account: TerminatedString,
        unknown2: u32,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        client_seed: [u8; CLIENT_SEED_SIZE],
        unknown3: u64,
        server_id: u32,
        unknown4: u64,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        digest: [u8; 20],
        addons_count: u32,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        addons: Vec<u8>,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { server_seed, .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        let (server_id, account, session_key, addons) = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            (
                guard.selected_realm.as_ref().unwrap().server_id as u32,
                config.connection_data.account.to_string(),
                guard.session_key.as_ref().unwrap().to_vec(),
                config.addons.clone()
            )
        };

        let client_seed: [u8; CLIENT_SEED_SIZE] = rand::random();

        let digest = Sha1::new()
            .chain(&account)
            .chain(vec![0, 0, 0, 0])
            .chain(client_seed)
            // from server_seed we need only CLIENT_SEED_SIZE first bytes
            .chain(&server_seed[..CLIENT_SEED_SIZE])
            .chain(session_key)
            .finalize()
            .to_vec();

        let addon_info = AddonInfo::build_addon_info(addons)?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&addon_info)?;

        response.push(HandlerOutput::Data(Outcome {
            build: 12340,
            unknown: 0,
            account: TerminatedString::from(account),
            unknown2: 0,
            client_seed,
            unknown3: 0,
            server_id,
            unknown4: 0,
            digest: digest.try_into().unwrap(),
            addons_count: addon_info.len() as u32,
            addons: encoder.finish()?,
        }.unpack()?));

        Ok(response)
    }
}