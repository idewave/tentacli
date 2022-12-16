use std::sync::mpsc::{self, Receiver, RecvError};

pub mod dialog;
pub mod flag;
pub mod event;
pub mod message;
pub mod types;

use crate::ipc::pipe::dialog::{DialogIncome, DialogOutcome};
use crate::ipc::pipe::flag::FlagOutcome;
use crate::ipc::pipe::event::EventIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::pipe::types::{IncomeMessageType, OutcomeMessageType};

pub struct IncomeMessagePipe {
    pub dialog_income: DialogIncome,
    pub message_income: MessageIncome,
    pub event_income: EventIncome,

    _receiver: Receiver<IncomeMessageType>,
}

impl IncomeMessagePipe {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel::<IncomeMessageType>();

        Self {
            dialog_income: DialogIncome::new(input_tx.clone()),
            message_income: MessageIncome::new(input_tx.clone()),
            event_income: EventIncome::new(input_tx),

            _receiver: input_rx,
        }
    }

    pub fn recv(&mut self) -> IncomeMessageType {
        match self._receiver.try_recv() {
            Ok(message) => message,
            _ => IncomeMessageType::None,
        }
    }
}

pub struct OutcomeMessagePipe {
    pub dialog_outcome: DialogOutcome,
    pub flag_outcome: FlagOutcome,
    _receiver: Receiver<OutcomeMessageType>,
}

impl OutcomeMessagePipe {
    pub fn new() -> Self {
        let (output_tx, output_rx) = mpsc::channel::<OutcomeMessageType>();

        Self {
            dialog_outcome: DialogOutcome::new(output_tx.clone()),
            flag_outcome: FlagOutcome::new(output_tx),
            _receiver: output_rx,
        }
    }

    pub fn recv(&mut self) -> Result<OutcomeMessageType, RecvError> {
        self._receiver.recv()
    }
}