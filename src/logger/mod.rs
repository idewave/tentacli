use std::sync::mpsc::{self, Receiver, Sender};

use crate::logger::types::LoggerOutput;

pub mod types;

// const BUFFER_SIZE: usize = 64;

pub struct LoggerChannel {
    pub output_sender: Sender<LoggerOutput>,
    _output_receiver: Receiver<LoggerOutput>,
}

impl LoggerChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<LoggerOutput>();

        Self {
            output_sender: tx,
            _output_receiver: rx,
        }
    }

    pub fn recv(&mut self) -> LoggerOutput {
        self._output_receiver.recv().unwrap()
    }
}