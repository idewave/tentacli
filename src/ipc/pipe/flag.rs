use std::sync::mpsc::Sender;

use crate::ipc::pipe::types::OutcomeMessageType;
use crate::ui::types::UIModeFlags;

#[derive(Clone, Debug)]
pub struct FlagOutcome {
    _sender: Sender<OutcomeMessageType>,
}

impl FlagOutcome {
    pub fn new(sender: Sender<OutcomeMessageType>) -> Self {
        Self {
            _sender: sender,
        }
    }

    pub fn send_toggle_flag(&mut self, flag: UIModeFlags) {
        self._sender.send(OutcomeMessageType::SetUIFlag(flag)).unwrap();
    }
}