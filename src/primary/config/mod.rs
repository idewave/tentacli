use std::env;
use std::path::Path;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::str::FromStr;
use anyhow::Error;
use yaml_rust::{Yaml, YamlLoader};

pub mod types;

use crate::primary::config::types::{AddonInfo, ChannelLabels, ConnectionData};
use crate::primary::errors::{ConfigError};

const CONFIG_CONTENT: &str = r##"connection_data:
  127.0.0.1:
    account_name:
        password: "safe_password"
        autoselect:
            realm_name: ".*STRING OR REGEX PATTERN TO FIND REALM NAME.*"
            character_name: ".*STRING OR REGEX PATTERN TO FIND CHARACTER NAME.*"

    another_account_name:
        password: "safe_password"
        autoselect:
            realm_name: ".*STRING OR REGEX PATTERN TO FIND REALM NAME.*"
            character_name: ".*STRING OR REGEX PATTERN TO FIND CHARACTER NAME.*"

  another.server.com:
    account_name:
        password: "safe_password"
        autoselect:
            realm_name: ""
            character_name: ""

channel_labels:
  lfg: "LFG"
  common: "COMMON"
  trade: "TRADE"
"##;

const ENV_CONFIG_CONTENT: &str = r##"CURRENT_HOST=127.0.0.1
CURRENT_PORT=3724
"##;

fn create_config_file(path: &str, content: &str) {
    if !Path::new(path).exists() {
        if let Some(parent_dir) = path.rfind('/') {
            let dirs_to_create = &path[..parent_dir];
            if !dirs_to_create.is_empty() {
                std::fs::create_dir_all(dirs_to_create).expect("Failed to create config directories");
            }
        }

        let mut file = File::create(path).expect("Failed to create YAML file");
        file.write_all(content.as_bytes()).unwrap();
    }
}

pub struct ConfigParams<'a> {
    pub host: &'a str,
    pub account: &'a str,
    pub config_path: &'a str,
}

pub struct EnvConfigParams<'a> {
    pub dotenv_path: &'a str,
}

pub struct Config {
    pub connection_data: ConnectionData,
    pub addons: Vec<AddonInfo>,
    pub channel_labels: ChannelLabels,
}

impl Config {
    pub fn new(params: ConfigParams) -> Result<Self, ConfigError> {
        create_config_file(params.config_path, CONFIG_CONTENT);

        let data = read_to_string(params.config_path).map_err(|_| ConfigError::NotFound)?;
        let docs = YamlLoader::load_from_str(&data).map_err(ConfigError::ScanError)?;

        let connection_data = Self::parse_connection_data(
            &docs[0]["connection_data"][params.host],
            params.account,
        );

        let channel_labels = Self::parse_channels_data(&docs[0]["channel_labels"]);

        Ok(Self {
            connection_data,
            addons: vec![
                AddonInfo { name: "Blizzard_AchievementUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_ArenaUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_AuctionUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_BarbershopUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_BattlefieldMinimap".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_BindingUI".to_string(), flags: 225, modulus_crc: 1276933997, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_Calendar".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_CombatLog".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_CombatText".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_DebugTools".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_GlyphUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_GMChatUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_GMSurveyUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_GuildBankUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_InspectUI".to_string(), flags: 92, modulus_crc: 1276933997, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_ItemSocketingUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_MacroUI".to_string(), flags: 31, modulus_crc: 1276933997, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_RaidUI".to_string(), flags: 201, modulus_crc: 1276933997, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_TalentUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_TimeManager".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_TokenUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_TradeSkillUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
                AddonInfo { name: "Blizzard_TrainerUI".to_string(), flags: 0, modulus_crc: 0, urlcrc_crc: 0 },
            ],
            channel_labels,
        })
    }

    fn parse_connection_data(config: &Yaml, account: &str) -> ConnectionData {
        let config = &config[account];
        let autoselect = config["autoselect"].as_hash().unwrap();

        return ConnectionData {
            account: account.to_string().to_uppercase(),
            password: config["password"].as_str().unwrap().to_string().to_uppercase(),
            autoselect_realm_name: autoselect
                .get(&Yaml::String("realm_name".to_string()))
                .unwrap()
                .as_str()
                .unwrap_or_default().to_string(),
            autoselect_character_name: autoselect
                .get(&Yaml::String("character_name".to_string()))
                .unwrap()
                .as_str()
                .unwrap_or_default().to_string(),
        }
    }

    fn parse_channels_data(config: &Yaml) -> ChannelLabels {
        return ChannelLabels {
            lfg: config["lfg"].as_str().unwrap().to_string(),
            common: config["common"].as_str().unwrap().to_string(),
            trade: config["trade"].as_str().unwrap().to_string(),
        }
    }
}

pub struct EnvConfig {
    pub host: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn new(params: EnvConfigParams) -> Result<Self, Error> {
        let dotenv_path = params.dotenv_path;
        create_config_file(dotenv_path, ENV_CONFIG_CONTENT);

        dotenv::from_filename(dotenv_path).ok();

        let host = env::var("CURRENT_HOST").expect("CURRENT_HOST must be set");
        let port = env::var("CURRENT_PORT").expect("CURRENT_PORT must be set");

        Ok(Self {
            host,
            port: u16::from_str(&port)?,
        })
    }
}
