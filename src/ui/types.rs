use std::sync::mpsc::Sender;
use bitflags::bitflags;
use crate::client::types::ClientFlags;

use crate::ipc::pipe::dialog::DialogOutcome;
use crate::ipc::pipe::flag::FlagOutcome;
use crate::ipc::pipe::types::IncomeMessageType;

pub struct UIRenderOptions<'a> {
    pub message: IncomeMessageType,
    pub client_flags: &'a ClientFlags,
}

#[derive(Clone)]
pub struct UIOutputOptions {
    pub dialog_outcome: DialogOutcome,
    pub flag_outcome: FlagOutcome,
}

#[derive(Clone)]
pub struct UIComponentOptions {
    pub output_options: UIOutputOptions,
    pub sender: Sender<String>,
}

bitflags! {
    pub struct UIStateFlags: u32 {
        const NONE = 0x00000000;
        const IS_CHARACTERS_MODAL_OPENED = 0x00000001;
        const IS_REALM_MODAL_OPENED = 0x00000010;
        const IS_EVENT_HANDLED = 0x00000100;
    }
}

bitflags! {
    pub struct UIModeFlags: u32 {
        const NONE = 0x00000000;
        const DEBUG_MODE = 0x00000001;
    }
}