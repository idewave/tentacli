pub struct TradeStatus;

#[allow(dead_code)]
impl TradeStatus {
    pub const TRADE_STATUS_BUSY: u8 = 0;
    pub const TRADE_STATUS_BEGIN_TRADE: u8 = 1;
    pub const TRADE_STATUS_OPEN_WINDOW: u8 = 2;
    pub const TRADE_STATUS_TRADE_CANCELED: u8 = 3;
    pub const TRADE_STATUS_TRADE_ACCEPT: u8 = 4;
    pub const TRADE_STATUS_BUSY_2: u8 = 5;
    pub const TRADE_STATUS_NO_TARGET: u8 = 6;
    pub const TRADE_STATUS_BACK_TO_TRADE: u8 = 7;
    pub const TRADE_STATUS_TRADE_COMPLETE: u8 = 8;
    pub const TRADE_STATUS_TRADE_REJECTED: u8 = 9;
    pub const TRADE_STATUS_TARGET_TO_FAR: u8 = 10;
    pub const TRADE_STATUS_WRONG_FACTION: u8 = 11;
    pub const TRADE_STATUS_CLOSE_WINDOW: u8 = 12;
    // 13?
    pub const TRADE_STATUS_IGNORE_YOU: u8 = 14;
    pub const TRADE_STATUS_YOU_STUNNED: u8 = 15;
    pub const TRADE_STATUS_TARGET_STUNNED: u8 = 16;
    pub const TRADE_STATUS_YOU_DEAD: u8 = 17;
    pub const TRADE_STATUS_TARGET_DEAD: u8 = 18;
    pub const TRADE_STATUS_YOU_LOGOUT: u8 = 19;
    pub const TRADE_STATUS_TARGET_LOGOUT: u8 = 20;
    pub const TRADE_STATUS_TRIAL_ACCOUNT: u8 = 21;
    pub const TRADE_STATUS_WRONG_REALM: u8 = 22;
    pub const TRADE_STATUS_NOT_ON_TAPLIST: u8 = 23;
}