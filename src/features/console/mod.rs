use async_broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tokio::task::JoinHandle;
use colored::*;

use crate::primary::traits::Feature;
use crate::primary::types::HandlerOutput;

pub struct Console {
    _receiver: BroadcastReceiver<HandlerOutput>,
    _sender: BroadcastSender<HandlerOutput>,
}

impl Feature for Console {
    fn new(
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>
    ) -> Self where Self: Sized {
        Self {
            _receiver: receiver,
            _sender: sender,
        }
    }

    fn get_tasks(&mut self) -> Vec<JoinHandle<()>> {
        let mut receiver = self._receiver.clone();

        let handle_input = || {
            tokio::spawn(async move {
                loop {
                    if let Ok(output) = receiver.recv().await {
                        match output {
                            HandlerOutput::SuccessMessage(message, _) => {
                                let text = format!("[SUCCESS]: {}", message);
                                println!("{}", text.bright_green());
                            },
                            HandlerOutput::ErrorMessage(message, _) => {
                                let text = format!("[ERROR]: {}", message);
                                println!("{}", text.bright_red());
                            },
                            HandlerOutput::DebugMessage(message, _) => {
                                let text = format!("[DEBUG]: {}", message);
                                println!("{}", text.bright_black());
                            },
                            HandlerOutput::ResponseMessage(message, _) => {
                                let text = format!("[RESPONSE]: {}", message);
                                println!("{}", text.bright_magenta());
                            },
                            HandlerOutput::RequestMessage(message, _) => {
                                let text = format!("[REQUEST]: {}", message);
                                println!("{}", text.bright_cyan());
                            },
                            _ => {},
                        }
                    }
                }
            })
        };

        vec![
            handle_input(),
        ]
    }
}