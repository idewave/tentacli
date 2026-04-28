use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::mem;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::login;
use crate::plugins::wow::wotlk::login::srp::Srp;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
pub struct Incoming {
    error: u8,
    server_proof: [u8; 20],
    account_flags: u32,
    survey_id: u32,
    unknown_flags: u16,
}

pub struct Handler {
    pub srp: Arc<Mutex<Srp>>,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut outputs = vec![];
        let Incoming { server_proof, .. } = Incoming::unpack(packet)?;

        let is_valid_proof = self
            .srp
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?
            .validate_proof(server_proof);

        if !is_valid_proof {
            outputs.extend([
                HandlerOutput::Messages(vec![Message {
                    msg_type: MsgType::Error,
                    text: "Proof is not valid".to_string(),
                }]),
                HandlerOutput::Requests(vec![Request::Drop(login::PLUGIN_LABEL)]),
            ]);
        } else {
            let session_key = {
                let mut guard = self
                    .srp
                    .lock()
                    .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
                mem::take(&mut guard.session_key)
            };

            outputs.extend([
                HandlerOutput::Messages(vec![Message {
                    msg_type: MsgType::Success,
                    text: "Proof is valid".to_string(),
                }]),
                HandlerOutput::Requests(vec![Request::SetContext(Some(Box::new(
                    move |ctx: &mut CtxMap| {
                        ctx.insert(Secret(session_key));
                    },
                )))]),
            ]);
        }

        Ok(outputs)
    }
}

pub struct Secret(pub Vec<u8>);
