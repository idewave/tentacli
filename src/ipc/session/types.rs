use bitflags::bitflags;

bitflags! {
    pub struct ActionFlags: u8 {
        const NONE = 0x00000000;
        const IS_CASTING = 0x00000001;
        const IS_FOLLOWING = 0x00000002;
        const IS_MOVING = 0x00000004;
    }
}

bitflags! {
    pub struct StateFlags: u8 {
        const NONE = 0x00000000;
        const IN_PARTY = 0x00000001;
        const IS_MOVEMENT_STARTED = 0x00000002;
    }
}