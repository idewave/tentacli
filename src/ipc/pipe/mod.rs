use std::sync::mpsc::{self, Receiver, RecvError};

pub mod dialog;
pub mod key_event;
pub mod message;
pub mod types;

use crate::ipc::pipe::dialog::{DialogIncome, DialogOutcome};
use crate::ipc::pipe::key_event::KeyEventIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::pipe::types::{IncomeMessageType, OutcomeMessageType};

pub struct IncomeMessagePipe {
    pub dialog_income: DialogIncome,
    pub message_income: MessageIncome,
    pub key_event_income: KeyEventIncome,

    _receiver: Receiver<IncomeMessageType>,
}

impl IncomeMessagePipe {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel::<IncomeMessageType>();

        Self {
            dialog_income: DialogIncome::new(input_tx.clone()),
            message_income: MessageIncome::new(input_tx.clone()),
            key_event_income: KeyEventIncome::new(input_tx.clone()),

            _receiver: input_rx,
        }
    }

    pub fn recv(&mut self) -> IncomeMessageType {
        self._receiver.recv().unwrap()
    }
}

pub struct OutcomeMessagePipe {
    pub dialog_outcome: DialogOutcome,
    _receiver: Receiver<OutcomeMessageType>,
}

impl OutcomeMessagePipe {
    pub fn new() -> Self {
        let (output_tx, output_rx) = mpsc::channel::<OutcomeMessageType>();

        Self {
            dialog_outcome: DialogOutcome::new(output_tx.clone()),
            _receiver: output_rx,
        }
    }

    pub fn recv(&mut self) -> Result<OutcomeMessageType, RecvError> {
        self._receiver.recv()
    }
}