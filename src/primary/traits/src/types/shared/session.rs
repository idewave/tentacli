use std::collections::HashSet;
use std::fmt::Debug;

use bitflags::bitflags;
use tentacli_crypto::Srp;

use crate::types::config::{Config, ConfigParams};
use crate::types::errors::ConfigError;
use crate::types::realm::Realm;
use crate::types::warden::WardenModuleInfo;

#[derive(Debug, Default)]
pub struct Session {
    pub srp: Option<Srp>,
    pub selected_realm: Option<Realm>,
    pub warden_module_info: Option<WardenModuleInfo>,
    pub config: Option<Config>,
    pub follow_target: Option<u64>,
    pub action_flags: ActionFlags,
    pub state_flags: StateFlags,
    pub spells_map: HashSet<u32>,
    pub my_guid: Option<u64>,
    pub inventory: Vec<i32>,
}

impl Session {
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