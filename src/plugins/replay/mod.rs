#![cfg(feature = "wow-wotlk")]
mod parser;

use std::collections::HashMap;
use std::sync::Arc;

use async_broadcast::Receiver;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

use crate::client::prelude::*;

const PLUGIN_LABEL: &str = crate::plugins::wow::wotlk::realm::PLUGIN_LABEL;

#[derive(Debug, Clone, Deserialize)]
pub struct ReplayCfg {
    pub world_log_path: String,
    pub opcodes: Vec<String>,
    pub dedup: bool,
    pub realtime: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub replay: Option<ReplayCfg>,
}

#[derive(Default)]
pub struct Replay;
impl Replay {
    async fn handle_connection(
        mut stream: TcpStream,
        shutdown: CancellationToken,
    ) -> anyhow::Result<()> {
        let config: Config = ConfigParser::parse_from_file("replay/replay.toml")?;
        let replay = config.replay.ok_or_else(|| anyhow::anyhow!("Missing [replay]"))?;

        let mut reader = parser::WorldLogReader::open(
            &replay.world_log_path,
            &replay.opcodes,
            replay.dedup
        )?;

        let mut first_log_timestamp: Option<i64> = None;
        let replay_start = Instant::now();

        loop {
            if shutdown.is_cancelled() {
                break;
            }

            let Some(packet_data) = reader.next_packet() else {
                break;
            };

            if replay.realtime {
                if let Some(ts) = packet_data.timestamp {
                    let base = first_log_timestamp.get_or_insert(ts);
                    let delta = ts.saturating_sub(*base) as u64;
                    let target = replay_start + Duration::from_secs(delta);

                    tokio::select! {
                        biased;
                        _ = shutdown.cancelled() => break,
                        _ = tokio::time::sleep_until(target) => {}
                    }
                }
            } else {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }

            let opcode = packet_data.opcode;
            let body = packet_data.payload;

            let size = (2 + body.len()) as u16;

            let mut buffer = Vec::with_capacity(2 + size as usize);
            buffer.extend_from_slice(&size.to_be_bytes());
            buffer.extend_from_slice(&opcode.to_le_bytes());
            buffer.extend_from_slice(&body);

            tokio::select! {
                biased;
                _ = shutdown.cancelled() => break,
                result = stream.write_all(&buffer) => {
                    result?;
                }
            }
        }

        Ok(())
    }

}

impl CorePlugin for Replay {
    fn get_tasks(
        &self,
        _broadcast_rx: Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        Ok(vec![
            tokio::spawn(async move {
                let listener = TcpListener::bind("127.0.0.1:0").await?;
                let local_addr = listener.local_addr()?;

                if let Some(sender) = echo_senders.get(PLUGIN_LABEL) {
                    sender.send(Echo::Connect(local_addr.to_string())).await?;
                } else {
                    anyhow::bail!("Echo sender for {PLUGIN_LABEL} not found");
                }

                let (stream, _) = tokio::select! {
                    biased;

                    _ = shutdown.cancelled() => {
                        return Ok(());
                    }

                    result = listener.accept() => {
                        result?
                    }
                };

                Replay::handle_connection(stream, shutdown.clone()).await?;

                Ok(())
            })
        ])
    }
}