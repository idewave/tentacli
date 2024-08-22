use std::collections::BTreeMap;
use tentacli_traits::Processor;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::ProcessorResult;

mod auth_challenge;

pub struct RealmProcessor;
impl Processor for RealmProcessor {
    fn get_one_time_handler_map() -> BTreeMap<u16, ProcessorResult> {
        let mut handlers_map: BTreeMap<u16, ProcessorResult> = BTreeMap::new();

        handlers_map.insert(Opcode::SMSG_AUTH_CHALLENGE, vec![
            Box::new(auth_challenge::Handler),
        ]);

        handlers_map
    }
}

pub mod packet {
    // Opcode::CMSG_LOGOUT_REQUEST
    #[derive(WorldPacket, Serialize, Debug, Default)]
    pub struct LogoutOutcoming {}
}