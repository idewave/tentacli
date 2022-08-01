use std::sync::mpsc::{self, Receiver, Sender};

pub mod dialog;
pub mod key_event;
pub mod message;
pub mod types;

use crate::ipc::duplex::dialog::{DialogIncome, DialogOutcome};
use crate::ipc::duplex::key_event::KeyEventIncome;
use crate::ipc::duplex::message::MessageIncome;
use crate::ipc::duplex::types::{IncomeMessageType, OutcomeMessageType};

pub struct MessageDuplex {
    pub dialog_income: DialogIncome,
    pub dialog_outcome: DialogOutcome,
    pub message_income: MessageIncome,
    pub key_event_income: KeyEventIncome,

    _income_receiver: Receiver<IncomeMessageType>,
    _outcome_receiver: Receiver<OutcomeMessageType>,
}

impl MessageDuplex {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel::<IncomeMessageType>();
        let (output_tx, output_rx) = mpsc::channel::<OutcomeMessageType>();

        Self {
            // from client to UI
            dialog_income: DialogIncome::new(input_tx.clone()),
            // from UI (dialog) to client
            dialog_outcome: DialogOutcome::new(output_tx.clone()),
            // from client to UI
            message_income: MessageIncome::new(input_tx.clone()),
            // from client to UI
            key_event_income: KeyEventIncome::new(input_tx.clone()),

            _income_receiver: input_rx,
            _outcome_receiver: output_rx,
        }
    }

    // messages from client to UI
    pub fn get_income(&mut self) -> IncomeMessageType {
        self._income_receiver.recv().unwrap()
    }

    // messages from UI to client
    pub fn get_outcome(&mut self) -> OutcomeMessageType {
        self._outcome_receiver.recv().unwrap()
    }
}