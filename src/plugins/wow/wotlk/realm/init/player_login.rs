use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use binrw::BinWrite;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_PLAYER_LOGIN)]
#[opcode(U32(Opcode::CMSG_PLAYER_LOGIN))]
#[bw(little)]
struct Outgoing {
    guid: u64,
}

pub struct Handler {
    pub guid: Arc<Mutex<u64>>,
    pub is_logged_in: Arc<Mutex<bool>>,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        _: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut output = vec![];
        let guid = *self.guid.lock().unwrap();
        let mut guard = self.is_logged_in.lock().unwrap();

        if *guard == false && guid > 0 {
            output.extend(vec![
                HandlerOutput::Packets(vec![
                    Outgoing { guid }.pack()?
                ]),
                HandlerOutput::Requests(vec![
                    Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                        ctx.insert(IsLogged(true));
                    }))),
                ])
            ]);

            *guard = true;
        }

        Ok(output)
    }
}

#[allow(dead_code)]
pub struct IsLogged(pub bool);