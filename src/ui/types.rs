use bitflags::bitflags;

use crate::ipc::pipe::dialog::DialogOutcome;
use crate::ipc::pipe::types::IncomeMessageType;

pub struct UIRenderOptions {
    pub message: IncomeMessageType,
}

pub struct UIOutputOptions {
    pub dialog_outcome: DialogOutcome,
}

bitflags! {
    pub struct UIStateFlags: u32 {
        const NONE = 0x00000000;
        const IS_CHARACTERS_MODAL_OPENED = 0x00000001;
        const IS_REALM_MODAL_OPENED = 0x00000010;
    }
}