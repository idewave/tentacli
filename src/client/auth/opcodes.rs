#[non_exhaustive]
pub struct Opcode;

impl Opcode {
    pub const LOGIN_CHALLENGE: u8 = 0x00;
    pub const LOGIN_PROOF: u8 = 0x01;
    pub const REALM_LIST: u8 = 0x10;
}