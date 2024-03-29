use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct ActionFlags: u8 {
        const NONE = 0x00000000;
        const IS_CASTING = 0x00000001;
        const IS_FOLLOWING = 0x00000002;
        const IS_MOVING = 0x00000004;
    }
}

bitflags! {
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct StateFlags: u32 {
        const NONE = 0x00000000;
        const IN_PARTY = 0x00000001;
        const IS_MOVEMENT_STARTED = 0x00000010;
        const IN_WORLD = 0x00000100;
    }
}