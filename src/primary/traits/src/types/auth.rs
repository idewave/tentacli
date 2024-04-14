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

#[non_exhaustive]
pub struct ResponseCodes;
#[allow(dead_code)]
impl ResponseCodes {
    pub const RESPONSE_SUCCESS: u8 = 0x00;
    pub const RESPONSE_FAILURE: u8 = 0x01;
    pub const RESPONSE_CANCELLED: u8 = 0x02;
    pub const RESPONSE_DISCONNECTED: u8 = 0x03;
    pub const RESPONSE_FAILED_TO_CONNECT: u8 = 0x04;
    pub const RESPONSE_CONNECTED: u8 = 0x05;
    pub const RESPONSE_VERSION_MISMATCH: u8 = 0x06;

    pub const CSTATUS_CONNECTING: u8 = 0x07;
    pub const CSTATUS_NEGOTIATING_SECURITY: u8 = 0x08;
    pub const CSTATUS_NEGOTIATION_COMPLETE: u8 = 0x09;
    pub const CSTATUS_NEGOTIATION_FAILED: u8 = 0x0A;
    pub const CSTATUS_AUTHENTICATING: u8 = 0x0B;

    pub const AUTH_OK: u8 = 0x0C;
    pub const AUTH_FAILED: u8 = 0x0D;
    pub const AUTH_REJECT: u8 = 0x0E;
    pub const AUTH_BAD_SERVER_PROOF: u8 = 0x0F;
    pub const AUTH_UNAVAILABLE: u8 = 0x10;
    pub const AUTH_SYSTEM_ERROR: u8 = 0x11;
    pub const AUTH_BILLING_ERROR: u8 = 0x12;
    pub const AUTH_BILLING_EXPIRED: u8 = 0x13;
    pub const AUTH_VERSION_MISMATCH: u8 = 0x14;
    pub const AUTH_UNKNOWN_ACCOUNT: u8 = 0x15;
    pub const AUTH_INCORRECT_PASSWORD: u8 = 0x16;
    pub const AUTH_SESSION_EXPIRED: u8 = 0x17;
    pub const AUTH_SERVER_SHUTTING_DOWN: u8 = 0x18;
    pub const AUTH_ALREADY_LOGGING_IN: u8 = 0x19;
    pub const AUTH_LOGIN_SERVER_NOT_FOUND: u8 = 0x1A;
    pub const AUTH_WAIT_QUEUE: u8 = 0x1B;
    pub const AUTH_BANNED: u8 = 0x1C;
    pub const AUTH_ALREADY_ONLINE: u8 = 0x1D;
    pub const AUTH_NO_TIME: u8 = 0x1E;
    pub const AUTH_DB_BUSY: u8 = 0x1F;
    pub const AUTH_SUSPENDED: u8 = 0x20;
    pub const AUTH_PARENTAL_CONTROL: u8 = 0x21;
    pub const AUTH_LOCKED_ENFORCED: u8 = 0x22;

    pub const REALM_LIST_IN_PROGRESS: u8 = 0x23;
    pub const REALM_LIST_SUCCESS: u8 = 0x24;
    pub const REALM_LIST_FAILED: u8 = 0x25;
    pub const REALM_LIST_INVALID: u8 = 0x26;
    pub const REALM_LIST_REALM_NOT_FOUND: u8 = 0x27;

    pub const ACCOUNT_CREATE_IN_PROGRESS: u8 = 0x28;
    pub const ACCOUNT_CREATE_SUCCESS: u8 = 0x29;
    pub const ACCOUNT_CREATE_FAILED: u8 = 0x2A;

    pub const CHAR_LIST_RETRIEVING: u8 = 0x2B;
    pub const CHAR_LIST_RETRIEVED: u8 = 0x2C;
    pub const CHAR_LIST_FAILED: u8 = 0x2D;

    pub const CHAR_CREATE_IN_PROGRESS: u8 = 0x2E;
    pub const CHAR_CREATE_SUCCESS: u8 = 0x2F;
    pub const CHAR_CREATE_ERROR: u8 = 0x30;
    pub const CHAR_CREATE_FAILED: u8 = 0x31;
    pub const CHAR_CREATE_NAME_IN_USE: u8 = 0x32;
    pub const CHAR_CREATE_DISABLED: u8 = 0x33;
    pub const CHAR_CREATE_PVP_TEAMS_VIOLATION: u8 = 0x34;
    pub const CHAR_CREATE_SERVER_LIMIT: u8 = 0x35;
    pub const CHAR_CREATE_ACCOUNT_LIMIT: u8 = 0x36;
    pub const CHAR_CREATE_SERVER_QUEUE: u8 = 0x37;
    pub const CHAR_CREATE_ONLY_EXISTING: u8 = 0x38;
    pub const CHAR_CREATE_EXPANSION: u8 = 0x39;
    pub const CHAR_CREATE_EXPANSION_CLASS: u8 = 0x3A;
    pub const CHAR_CREATE_LEVEL_REQUIREMENT: u8 = 0x3B;
    pub const CHAR_CREATE_UNIQUE_CLASS_LIMIT: u8 = 0x3C;
    pub const CHAR_CREATE_CHARACTER_IN_GUILD: u8 = 0x3D;
    pub const CHAR_CREATE_RESTRICTED_RACECLASS: u8 = 0x3E;
    pub const CHAR_CREATE_CHARACTER_CHOOSE_RACE: u8 = 0x3F;
    pub const CHAR_CREATE_CHARACTER_ARENA_LEADER: u8 = 0x40;
    pub const CHAR_CREATE_CHARACTER_DELETE_MAIL: u8 = 0x41;
    pub const CHAR_CREATE_CHARACTER_SWAP_FACTION: u8 = 0x42;
    pub const CHAR_CREATE_CHARACTER_RACE_ONLY: u8 = 0x43;
    pub const CHAR_CREATE_CHARACTER_GOLD_LIMIT: u8 = 0x44;
    pub const CHAR_CREATE_FORCE_LOGIN: u8 = 0x45;

    pub const CHAR_DELETE_IN_PROGRESS: u8 = 0x46;
    pub const CHAR_DELETE_SUCCESS: u8 = 0x47;
    pub const CHAR_DELETE_FAILED: u8 = 0x48;
    pub const CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER: u8 = 0x49;
    pub const CHAR_DELETE_FAILED_GUILD_LEADER: u8 = 0x4A;
    pub const CHAR_DELETE_FAILED_ARENA_CAPTAIN: u8 = 0x4B;

    pub const CHAR_LOGIN_IN_PROGRESS: u8 = 0x4C;
    pub const CHAR_LOGIN_SUCCESS: u8 = 0x4D;
    pub const CHAR_LOGIN_NO_WORLD: u8 = 0x4E;
    pub const CHAR_LOGIN_DUPLICATE_CHARACTER: u8 = 0x4F;
    pub const CHAR_LOGIN_NO_INSTANCES: u8 = 0x50;
    pub const CHAR_LOGIN_FAILED: u8 = 0x51;
    pub const CHAR_LOGIN_DISABLED: u8 = 0x52;
    pub const CHAR_LOGIN_NO_CHARACTER: u8 = 0x53;
    pub const CHAR_LOGIN_LOCKED_FOR_TRANSFER: u8 = 0x54;
    pub const CHAR_LOGIN_LOCKED_BY_BILLING: u8 = 0x55;
    pub const CHAR_LOGIN_LOCKED_BY_MOBILE_AH: u8 = 0x56;

    pub const CHAR_NAME_SUCCESS: u8 = 0x57;
    pub const CHAR_NAME_FAILURE: u8 = 0x58;
    pub const CHAR_NAME_NO_NAME: u8 = 0x59;
    pub const CHAR_NAME_TOO_SHORT: u8 = 0x5A;
    pub const CHAR_NAME_TOO_LONG: u8 = 0x5B;
    pub const CHAR_NAME_INVALID_CHARACTER: u8 = 0x5C;
    pub const CHAR_NAME_MIXED_LANGUAGES: u8 = 0x5D;
    pub const CHAR_NAME_PROFANE: u8 = 0x5E;
    pub const CHAR_NAME_RESERVED: u8 = 0x5F;
    pub const CHAR_NAME_INVALID_APOSTROPHE: u8 = 0x60;
    pub const CHAR_NAME_MULTIPLE_APOSTROPHES: u8 = 0x61;
    pub const CHAR_NAME_THREE_CONSECUTIVE: u8 = 0x62;
    pub const CHAR_NAME_INVALID_SPACE: u8 = 0x63;
    pub const CHAR_NAME_CONSECUTIVE_SPACES: u8 = 0x64;
    pub const CHAR_NAME_RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS: u8 = 0x65;
    pub const CHAR_NAME_RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END: u8 = 0x66;
    pub const CHAR_NAME_DECLENSION_DOESNT_MATCH_BASE_NAME: u8 = 0x67;
}

#[non_exhaustive]
pub struct Expansions;
#[allow(dead_code)]
impl Expansions {
    pub const EXPANSION_CLASSIC: u8 = 0;
    pub const EXPANSION_TBC: u8 = 1;
    pub const EXPANSION_WOTLK: u8 = 2;
}