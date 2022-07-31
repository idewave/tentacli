use std::fmt::{Debug, Formatter};

pub mod types;

use crate::client::{Player, WardenModuleInfo};
use crate::config::{Config, ConfigParams};
use crate::ipc::session::types::{ActionFlags, StateFlags};

pub struct Session {
    pub session_key: Option<Vec<u8>>,
    pub server_id: Option<u8>,
    pub warden_module_info: Option<WardenModuleInfo>,
    pub config: Option<Config>,
    pub me: Option<Player>,
    pub follow_target: Option<u64>,
    pub action_flags: ActionFlags,
    pub state_flags: StateFlags,
    pub party: Vec<Player>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            session_key: None,
            server_id: None,
            warden_module_info: None,
            config: None,
            me: None,
            follow_target: None,
            action_flags: ActionFlags::NONE,
            state_flags: StateFlags::NONE,
            party: Vec::new(),
        }
    }

    pub fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn set_config(&mut self, host: &str) {
        if self.config.is_none() {
            let config = Config::new(ConfigParams {
                host,
            }).unwrap();

            self.config = Some(config);
        }
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nsession_key: {:?}, server_id: '{:?}'",
            self.session_key,
            self.server_id,
        )
    }
}