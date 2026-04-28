//! Optional WebSocket broadcast plugin.
//!
//! This core plugin broadcasts `HandlerOutput` events to all connected
//! WebSocket clients as JSON.

use std::collections::HashMap;
use std::sync::Arc;

use async_broadcast::Receiver;
use futures::{SinkExt, StreamExt};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{RwLock, broadcast, mpsc::Sender};
use tokio::time::{Duration, timeout};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};
use tokio_util::sync::CancellationToken;

use crate::client::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub local: bool,
}

struct FilteredOutputs<'a> {
    outputs: &'a [HandlerOutput],
}

impl Serialize for FilteredOutputs<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        for output in self.outputs {
            match output {
                HandlerOutput::Messages(messages) => {
                    seq.serialize_element(&WsOutputRef::Messages(messages))?;
                }
                HandlerOutput::Packets(packets) => {
                    seq.serialize_element(&WsOutputRef::Packets(packets))?;
                }
                HandlerOutput::Requests(_) => {}
            }
        }

        seq.end()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
enum WsOutputRef<'a> {
    Messages(&'a [Message]),
    Packets(&'a [Packet]),
}

#[derive(Serialize)]
struct WsEvent<'a> {
    label: &'a str,
    outputs: FilteredOutputs<'a>,
}

#[derive(Default)]
pub struct WebSocket;

impl WebSocket {
    async fn handle_connection(
        stream: TcpStream,
        mut event_rx: broadcast::Receiver<Arc<str>>,
        shutdown: CancellationToken,
    ) -> anyhow::Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (mut sink, mut source) = ws_stream.split();

        loop {
            tokio::select! {
                biased;

                _ = shutdown.cancelled() => break,

                event = event_rx.recv() => {
                    match event {
                        Ok(payload) => {
                            let send = sink.send(WsMessage::Text(payload.as_ref().into()));
                            match timeout(Duration::from_secs(5), send).await {
                                Ok(Ok(())) => {}
                                _ => break,
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }

                incoming = source.next() => {
                    match incoming {
                        Some(Ok(WsMessage::Close(_))) | None => break,
                        Some(Ok(_)) => {}
                        Some(Err(err)) => return Err(err.into()),
                    }
                }
            }
        }

        Ok(())
    }
}

impl CorePlugin for WebSocket {
    fn get_tasks(
        &self,
        broadcast_rx: Receiver<OrderedOutput>,
        _echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        let config: Config = ConfigParser::parse_from_file("websocket/websocket.toml")?;
        let bind_host = if config.local { "127.0.0.1" } else { "0.0.0.0" };

        Ok(vec![tokio::spawn(async move {
            let listener = TcpListener::bind((bind_host, config.port)).await?;
            let (event_tx, _) = broadcast::channel::<Arc<str>>(1024);
            let mut ordered_rx = OrderedReceiver::new(broadcast_rx);

            loop {
                tokio::select! {
                    biased;

                    _ = shutdown.cancelled() => break,

                    accepted = listener.accept() => {
                        let (stream, _) = accepted?;
                        let client_shutdown = shutdown.child_token();
                        let client_rx = event_tx.subscribe();

                        tokio::spawn(async move {
                            let _ = Self::handle_connection(stream, client_rx, client_shutdown).await;
                        });
                    }

                    next = ordered_rx.recv() => {
                        let OrderedOutput { label, outputs, .. } = next?;

                        if outputs.iter().all(|o| matches!(o, HandlerOutput::Requests(_))) {
                            continue;
                        }

                        let event = WsEvent {
                            label,
                            outputs: FilteredOutputs { outputs: outputs.as_ref() },
                        };

                        let payload: Arc<str> = serde_json::to_string(&event)?.into();
                        let _ = event_tx.send(payload);
                    }
                }
            }

            Ok(())
        })])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that only TUI-visible outputs (`Messages`, `Packets`) are
    /// forwarded, while `Requests` are ignored.
    #[test]
    fn filtered_outputs_ignores_requests() {
        let outputs = vec![
            HandlerOutput::Requests(vec![Request::Drop("realm")]),
            HandlerOutput::Messages(vec![Message {
                msg_type: MsgType::Info,
                text: "hello".to_string(),
            }]),
            HandlerOutput::Packets(vec![Packet::default()]),
        ];

        let json = serde_json::to_string(&WsEvent {
            label: "realm",
            outputs: FilteredOutputs { outputs: &outputs },
        })
        .expect("event should serialize");

        assert!(json.contains("\"Messages\""));
        assert!(json.contains("\"Packets\""));
        assert!(!json.contains("\"Requests\""));
    }

    /// Ensures the plugin config can be parsed from TOML and exposes the
    /// required WebSocket server settings.
    #[test]
    fn websocket_config_parses_port_and_local() {
        let config: Config =
            ConfigParser::parse_from_string("port = 9001\nlocal = true\n".to_string())
                .expect("websocket config should parse");

        assert_eq!(config.port, 9001);
        assert!(config.local);
    }
}
