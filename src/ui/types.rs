use bitflags::bitflags;

use crate::ipc::duplex::types::IncomeMessageType;

pub struct UIOptions {
    pub message: IncomeMessageType,
}

bitflags! {
    pub struct UIStateFlags: u32 {
        const NONE = 0x00000000;
        const IS_CHARACTERS_MODAL_OPENED = 0x00000001;
        const IS_REALM_MODAL_OPENED = 0x00000010;
    }
}