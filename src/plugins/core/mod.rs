use std::collections::HashMap;
use std::sync::Arc;
use async_broadcast::{Receiver, RecvError};
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::client::prelude::*;

#[derive(Default)]
pub struct Core;
impl CorePlugin for Core {
    fn get_tasks(
        &self,
        broadcast_rx: Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        let handle_requests = tokio::spawn(async move {
            let mut ordered_rx = OrderedReceiver::new(broadcast_rx.clone());
            loop {
                tokio::select! {
                    biased;

                    _ = shutdown.cancelled() => {
                        break;
                    },

                    result = ordered_rx.recv() => {
                        match result {
                            Ok(ordered) => {
                                for output in ordered.outputs.iter() {
                                    if let HandlerOutput::Requests(requests) = output {
                                        for req in requests {
                                            match req {
                                                Request::Connect(label, addr) => {
                                                    if let Some(sender) = echo_senders.get(label) {
                                                        sender.send(
                                                            Echo::Connect(addr.to_string())
                                                        ).await?;
                                                    }
                                                },
                                                Request::Drop(label) => {
                                                    if let Some(sender) = echo_senders.get(label) {
                                                        sender.send(Echo::Drop).await?;
                                                    }
                                                },
                                                _ => {},
                                            }
                                        }
                                    }
                                }
                            },
                            Err(RecvError::Overflowed(_)) => {
                                continue;
                            }
                            Err(RecvError::Closed) => { break; }
                        }
                    }
                }
            }

            Ok(())
        });

        Ok(vec![handle_requests])
    }
}