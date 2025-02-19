use async_broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver};
use colored::*;
use tentacli_traits::{Feature, FeatureError};
use tentacli_traits::types::{HandlerOutput, Task};

#[derive(Default)]
pub struct Console {
    _receiver: Option<BroadcastReceiver<HandlerOutput>>,
    _sender: Option<BroadcastSender<HandlerOutput>>,
}

impl Feature for Console {
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>
    ) {
        self._sender = Some(sender);
        self._receiver = Some(receiver);
    }

    fn get_tasks(&mut self) -> anyhow::Result<Vec<Task>> {
        let mut receiver = self._receiver.as_mut().ok_or(FeatureError::ReceiverNotFound)?.clone();

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
                                let text = format!("[RECV]: {}", message);
                                println!("{}", text.bright_magenta());
                            },
                            HandlerOutput::RequestMessage(message, _) => {
                                let text = format!("[SEND]: {}", message);
                                println!("{}", text.bright_cyan());
                            },
                            _ => {},
                        }
                    }
                }
            })
        };

        Ok(vec![
            handle_input(),
        ])
    }
}