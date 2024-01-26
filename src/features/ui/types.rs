use bitflags::bitflags;

#[derive(Debug)]
pub enum LoggerOutput {
    // title, optional details
    Debug(String, Option<String>),
    Error(String, Option<String>),
    Success(String, Option<String>),
    Response(String, Option<String>),
    Request(String, Option<String>),
}

bitflags! {
    pub struct UIEventFlags: u32 {
        const NONE = 0x00000000;
        const IS_CHARACTERS_MODAL_OPENED = 0x00000001;
        const IS_REALM_MODAL_OPENED = 0x00000010;
        const IS_EVENT_HANDLED = 0x00000100;
    }
}