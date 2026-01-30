use std::collections::HashMap;
use std::sync::Arc;
use async_broadcast::Receiver;
use colored::Colorize;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::client::prelude::*;

/// Plugin designed for debug purposes
/// It makes it simple to put println! whenever you need
/// Supports input (for Request::InitChoice etc)
/// To use enable dbg-ui feature
#[derive(Default)]
pub struct DbgUI;
impl CorePlugin for DbgUI {
    fn get_tasks(
        &self,
        broadcast_rx: Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        _: CancellationToken,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<Task>> {
        let handle_io = tokio::spawn(async move {
            let mut ordered_rx = OrderedReceiver::new(broadcast_rx);
            let stdin = BufReader::new(io::stdin());
            let mut lines = stdin.lines();

            loop {
                if let Ok(OrderedOutput { label, outputs, .. }) = ordered_rx.recv().await {
                    for output in &*outputs {
                        match output {
                            HandlerOutput::Packets(packets) => {
                                for packet in packets {
                                    let is_incoming = {
                                        packet.metadata.packet_type == PacketType::Incoming
                                    };

                                    if is_incoming {
                                        let text = format!(
                                            "[RECV] {}",
                                            packet.metadata.packet_name
                                        );

                                        println!("{}", text.bright_magenta());
                                    } else {
                                        let text = format!(
                                            "[SENT] {}",
                                            packet.metadata.packet_name
                                        );

                                        println!("{}", text.bright_cyan());
                                    }
                                }
                            },
                            HandlerOutput::Messages(messages) => {
                                for message in messages {
                                    match message.msg_type {
                                        MsgType::Success => {
                                            println!("{}", message.text.bright_green());
                                        },
                                        MsgType::Error => {
                                            println!("{}", message.text.bright_red());
                                        },
                                        MsgType::Info => {
                                            println!("{}", message.text.bright_black());
                                        },
                                    }
                                }
                            },
                            HandlerOutput::Requests(requests) => {
                                for request in requests {
                                    if let Request::InitChoice(items, _) = request {
                                        println!(
                                            "{}",
                                            "[Select] Please choose \
                                            (input number and press Enter):".bright_yellow()
                                        );

                                        for (i, item) in items.iter().enumerate() {
                                            println!(
                                                "●  {}: {}",
                                                i.to_string().bright_yellow(),
                                                item.purple()
                                            );
                                        }

                                        loop {
                                            if let Ok(Some(line)) = lines.next_line().await {
                                                if let Ok(idx) = line.trim().parse::<usize>() {
                                                    if let Some(chosen) = items.get(idx) {
                                                        println!("You chose: {}", chosen);

                                                        let option = echo_senders.get(&label);
                                                        if let Some(sender) = option {
                                                            let _ = sender.send(
                                                                Echo::Choose(vec![idx])
                                                            ).await;
                                                        }

                                                        break;
                                                    } else {
                                                        println!(
                                                            "{}",
                                                            "Invalid index, try again".bright_red()
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        });

        Ok(vec![handle_io])
    }
}