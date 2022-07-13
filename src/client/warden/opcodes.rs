#[non_exhaustive]
pub struct WardenOpcode;

#[allow(dead_code)]
impl WardenOpcode {
    pub const WARDEN_CMSG_MODULE_MISSING: u8 = 0;
    pub const WARDEN_CMSG_MODULE_OK: u8 = 1;
    pub const WARDEN_CMSG_CHEAT_CHECKS_RESULT: u8 = 2;
    pub const WARDEN_CMSG_MEM_CHECKS_RESULT: u8 = 3;
    pub const WARDEN_CMSG_HASH_RESULT: u8 = 4;
    pub const WARDEN_CMSG_MODULE_FAILED: u8 = 5;

    pub const WARDEN_SMSG_MODULE_USE: u8 = 0;
    pub const WARDEN_SMSG_MODULE_CACHE: u8 = 1;
    pub const WARDEN_SMSG_CHEAT_CHECKS_REQUEST: u8 = 2;
    pub const WARDEN_SMSG_MODULE_INITIALIZE: u8 = 3;
    pub const WARDEN_SMSG_MEM_CHECKS_REQUEST: u8 = 4;
    pub const WARDEN_SMSG_HASH_REQUEST: u8 = 5;
}