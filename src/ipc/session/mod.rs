use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::io::Error;

pub mod types;

use crate::client::{Player, Realm, WardenModuleInfo};
use crate::config::{Config, ConfigParams};
use crate::ipc::session::types::{ActionFlags, StateFlags};

pub struct Session {
    pub session_key: Option<Vec<u8>>,
    pub selected_realm: Option<Realm>,
    pub warden_module_info: Option<WardenModuleInfo>,
    pub config: Option<Config>,
    pub me: Option<Player>,
    pub follow_target: Option<u64>,
    pub action_flags: ActionFlags,
    pub state_flags: StateFlags,
    pub party: Vec<Player>,
    pub spells_map: HashSet<u32>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            session_key: None,
            selected_realm: None,
            warden_module_info: None,
            config: None,
            me: None,
            follow_target: None,
            action_flags: ActionFlags::NONE,
            state_flags: StateFlags::NONE,
            party: Vec::new(),
            spells_map: HashSet::new(),
        }
    }

    pub fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn set_config(&mut self, host: &str) -> Result<(), Error> {
        if self.config.is_none() {
            let result = Config::new(ConfigParams {
                host,
            });

            return match result {
                Ok(config) => {
                    self.config = Some(config);
                    Ok(())
                },
                Err(err) => {
                    Err(err)
                },
            }
        }

        Ok(())
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "session_key: {:?}",
            self.session_key,
        )
    }
}