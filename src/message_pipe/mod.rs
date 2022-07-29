use std::sync::mpsc::{self, Receiver, Sender};
use crate::message_pipe::dialog_sender::DialogSender;
use crate::message_pipe::message_sender::MessageSender;

use crate::message_pipe::types::MessageType;

pub mod dialog_sender;
pub mod message_sender;
pub mod types;

pub struct MessagePipe {
    pub dialog_sender: DialogSender,
    pub message_sender: MessageSender,
    _output_receiver: Receiver<MessageType>,
}

impl MessagePipe {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<MessageType>();

        Self {
            dialog_sender: DialogSender::new(tx.clone()),
            message_sender: MessageSender::new(tx.clone()),
            _output_receiver: rx,
        }
    }

    pub fn recv(&mut self) -> MessageType {
        self._output_receiver.recv().unwrap()
    }
}