use async_broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tokio::task::JoinHandle;

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
                            HandlerOutput::SuccessMessage(message, _)
                            | HandlerOutput::ErrorMessage(message, _)
                            | HandlerOutput::DebugMessage(message, _)
                            | HandlerOutput::ResponseMessage(message, _)
                            | HandlerOutput::RequestMessage(message, _) => {
                                println!("{}", message);
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