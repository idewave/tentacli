use bitflags::bitflags;

bitflags! {
    pub struct ClientFlags: u32 {
        const NONE = 0x00000000;
        const IS_CONNECTED_TO_REALM = 0x00000001;
        const IN_DEBUG_MODE = 0x00000010;
    }
}