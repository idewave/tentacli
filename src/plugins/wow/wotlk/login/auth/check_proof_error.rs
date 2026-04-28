use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::login;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
pub struct Incoming {
    unknown: u8,
    code: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut outputs = vec![];
        let Incoming { code, .. } = Incoming::unpack(packet)?;

        if code != Codes::SUCCESS {
            let text = match code {
                Codes::FAILED_UNKNOWN0
                | Codes::FAILED_UNKNOWN1
                | Codes::FAILED_INVALID_SERVER
                | Codes::FAILED_FAIL_NOACCESS => "Unable to connect".to_string(),
                Codes::FAILED_BANNED => "Account was banned".to_string(),
                Codes::FAILED_SUSPENDED => "Account was temporary suspended".to_string(),
                Codes::FAILED_UNKNOWN_ACCOUNT | Codes::FAILED_INCORRECT_PASSWORD => {
                    "Credentials not valid".to_string()
                }
                Codes::FAILED_ALREADY_ONLINE => "Account already online".to_string(),
                Codes::FAILED_DB_BUSY => "Cannot login at this time, try again later".to_string(),
                _ => {
                    format!("Unknown error with code: \"{code}\"")
                }
            };

            outputs.extend([
                HandlerOutput::Messages(vec![Message {
                    msg_type: MsgType::Error,
                    text,
                }]),
                HandlerOutput::Requests(vec![Request::Drop(login::PLUGIN_LABEL)]),
            ])
        }

        Ok(outputs)
    }
}

#[non_exhaustive]
pub struct Codes;
#[allow(dead_code)]
impl Codes {
    pub const SUCCESS: u8 = 0x00;
    pub const FAILED_UNKNOWN0: u8 = 0x01;
    pub const FAILED_UNKNOWN1: u8 = 0x02;
    pub const FAILED_BANNED: u8 = 0x03;
    pub const FAILED_UNKNOWN_ACCOUNT: u8 = 0x04;
    pub const FAILED_INCORRECT_PASSWORD: u8 = 0x05;
    pub const FAILED_ALREADY_ONLINE: u8 = 0x06;
    pub const FAILED_NO_TIME: u8 = 0x07;
    pub const FAILED_DB_BUSY: u8 = 0x08;
    pub const FAILED_VERSION_INVALID: u8 = 0x09;
    pub const FAILED_VERSION_UPDATE: u8 = 0x0A;
    pub const FAILED_INVALID_SERVER: u8 = 0x0B;
    pub const FAILED_SUSPENDED: u8 = 0x0C;
    pub const FAILED_FAIL_NOACCESS: u8 = 0x0D;
    pub const SUCCESS_SURVEY: u8 = 0x0E;
    pub const FAILED_PARENTCONTROL: u8 = 0x0F;
    pub const FAILED_LOCKED_ENFORCED: u8 = 0x10;
    pub const FAILED_TRIAL_ENDED: u8 = 0x11;
    pub const FAILED_USE_BNET: u8 = 0x12;
}
