#[non_exhaustive]
pub struct AuthLogonResult;

#[allow(dead_code)]
impl AuthLogonResult {
    pub const AUTH_LOGON_SUCCESS: u8 = 0x00;
    pub const AUTH_LOGON_FAILED_UNKNOWN0: u8 = 0x01;
    pub const AUTH_LOGON_FAILED_UNKNOWN1: u8 = 0x02;
    pub const AUTH_LOGON_FAILED_BANNED: u8 = 0x03;
    pub const AUTH_LOGON_FAILED_UNKNOWN_ACCOUNT: u8 = 0x04;
    pub const AUTH_LOGON_FAILED_INCORRECT_PASSWORD: u8 = 0x05;
    pub const AUTH_LOGON_FAILED_ALREADY_ONLINE: u8 = 0x06;
    pub const AUTH_LOGON_FAILED_NO_TIME: u8 = 0x07;
    pub const AUTH_LOGON_FAILED_DB_BUSY: u8 = 0x08;
    pub const AUTH_LOGON_FAILED_VERSION_INVALID: u8 = 0x09;
    pub const AUTH_LOGON_FAILED_VERSION_UPDATE: u8 = 0x0A;
    pub const AUTH_LOGON_FAILED_INVALID_SERVER: u8 = 0x0B;
    pub const AUTH_LOGON_FAILED_SUSPENDED: u8 = 0x0C;
    pub const AUTH_LOGON_FAILED_FAIL_NOACCESS: u8 = 0x0D;
    pub const AUTH_LOGON_SUCCESS_SURVEY: u8 = 0x0E;
    pub const AUTH_LOGON_FAILED_PARENTCONTROL: u8 = 0x0F;
    pub const AUTH_LOGON_FAILED_LOCKED_ENFORCED: u8 = 0x10;
    pub const AUTH_LOGON_FAILED_TRIAL_ENDED: u8 = 0x11;
    pub const AUTH_LOGON_FAILED_USE_BNET: u8 = 0x12;
}