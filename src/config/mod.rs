use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use yaml_rust::{Yaml, YamlLoader};

mod types;

use crate::config::types::{AddonInfo, BotChat, ConnectionData};

pub struct ConfigParams<'a> {
    pub host: &'a str,
}

pub struct Config {
    pub connection_data: ConnectionData,
    pub addons: Vec<AddonInfo>,
    pub bot_chat: BotChat,
}

impl Config {
    pub fn new(params: ConfigParams) -> Result<Self, Error> {
        if let Ok(data) = read_to_string("Config.yml") {
            let docs = YamlLoader::load_from_str(&data).unwrap();

            let connection_data = Self::parse_connection_data(
                &docs[0]["connection_data"][params.host]
            );
            let addons = Self::parse_addons(&docs[0]["addons"]);
            let bot_chat = Self::parse_chat_config(&docs[0]["bot_chat"]);

            return Ok(Self {
                connection_data,
                addons,
                bot_chat,
            });
        }

        Err(Error::new(ErrorKind::NotFound, "Config.yml file is not found"))
    }

    fn parse_connection_data(config: &Yaml) -> ConnectionData {
        return ConnectionData {
            username: config["username"].as_str().unwrap().to_string().to_uppercase(),
            password: config["password"].as_str().unwrap().to_string().to_uppercase(),
            realm_name: config["realm_name"].as_str().unwrap().to_string(),
        }
    }

    fn parse_addons(config: &Yaml) -> Vec<AddonInfo> {
        let mut addons: Vec<AddonInfo> = Vec::new();

        for addon in config.as_vec().unwrap() {
            addons.push(AddonInfo {
                name: addon["name"].as_str().unwrap().to_string(),
                flags: addon["flags"].as_i64().unwrap() as u8,
                modulus_crc: addon["modulus_crc"].as_i64().unwrap() as u32,
                urlcrc_crc: addon["urlcrc_crc"].as_i64().unwrap() as u32,
            });
        }

        addons
    }

    fn parse_chat_config(config: &Yaml) -> BotChat {
        let greet_messages = config["greet_messages"].as_vec().unwrap();
        let agree = config["agree"].as_vec().unwrap();
        let disagree = config["disagree"].as_vec().unwrap();
        let follow_invite = config["follow_invite"].as_vec().unwrap();
        let stop = config["stop"].as_vec().unwrap();

        return BotChat {
            greet: greet_messages
                .iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            agree: agree
                .iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            disagree: disagree
                .iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            follow_invite: follow_invite
                .iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            stop: stop
                .iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        }
    }
}
