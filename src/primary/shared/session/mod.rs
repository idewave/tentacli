use std::collections::HashSet;
use std::fmt::{Debug};

pub mod types;

use crate::primary::client::{Player, Realm, WardenModuleInfo};
use crate::primary::config::{Config, ConfigParams};
use crate::primary::crypto::srp::Srp;
use crate::primary::errors::ConfigError;
use crate::primary::shared::session::types::{ActionFlags, StateFlags};

#[derive(Debug)]
pub struct Session {
    pub srp: Option<Srp>,
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
            srp: None,
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

    pub fn get_config(&self) -> Result<&Config, ConfigError> {
        self.config.as_ref().ok_or(ConfigError::NotFound)
    }

    pub fn set_config(&mut self, host: &str, account: &str, config_path: &str) -> Result<(), ConfigError> {
        if self.config.is_none() {
            let config = Config::new(ConfigParams {
                host,
                account,
                config_path,
            })?;

            self.config = Some(config);
        }

        Ok(())
    }
}