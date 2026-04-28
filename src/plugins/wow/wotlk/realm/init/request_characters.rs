use async_trait::async_trait;
use binrw::BinWrite;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata, Default)]
#[name(CMSG_CHAR_ENUM)]
#[opcode(U32(Opcode::CMSG_CHAR_ENUM))]
#[bw(little)]
pub struct Outgoing {}

pub struct Handler {
    pub guid: Arc<Mutex<u64>>,
    pub is_logged_in: Arc<Mutex<bool>>,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        _: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut output = vec![];
        let guid = *self.guid.lock().unwrap();
        let is_logged_in = *self.is_logged_in.lock().unwrap();

        if guid == 0 && !is_logged_in {
            output.extend(vec![HandlerOutput::Packets(vec![
                Outgoing::default().pack()?,
            ])]);
        }
        Ok(output)
    }
}
