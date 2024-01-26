use async_broadcast::{Sender as BroadcastSender};
use bitflags::bitflags;

use crate::primary::types::HandlerOutput;

#[derive(Clone)]
pub struct UIComponentOptions {
    pub sender: BroadcastSender<HandlerOutput>,
}

bitflags! {
    pub struct UIEventFlags: u32 {
        const NONE = 0x00000000;
        const IS_CHARACTERS_MODAL_OPENED = 0x00000001;
        const IS_REALM_MODAL_OPENED = 0x00000010;
        const IS_EVENT_HANDLED = 0x00000100;
    }
}