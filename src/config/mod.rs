use std::fs::read_to_string;
use std::io::{Error};
use yaml_rust::{Yaml, YamlLoader};

pub struct ConnectionData {
    pub username: String,
    pub password: String,
    pub realm_name: String,
}

pub struct BotChat {
    pub greet: Vec<String>,
    pub agree: Vec<String>,
    pub disagree: Vec<String>,
    pub follow_invite: Vec<String>,
    pub stop: Vec<String>,
}

pub struct ConfigParams<'a> {
    pub host: &'a str,
}

pub struct Config {
    pub connection_data: ConnectionData,
    pub addons: Yaml,
    pub bot_chat: BotChat,
}

impl Config {
    pub fn new(params: ConfigParams) -> Result<Self, Error> {
        let data = read_to_string("Config.yml").unwrap();
        let docs = YamlLoader::load_from_str(&data).unwrap();

        let connection_data = Self::parse_connection_data(&docs[0]["connection_data"][params.host]);
        let addons = &docs[0]["addons"];
        let bot_chat = Self::parse_chat_config(&docs[0]["bot_chat"]);

        Ok(Self {
            connection_data,
            addons: addons.to_owned(),
            bot_chat,
        })
    }

    fn parse_connection_data(config: &Yaml) -> ConnectionData {
        return ConnectionData {
            username: config["username"].as_str().unwrap().to_string().to_uppercase(),
            password: config["password"].as_str().unwrap().to_string().to_uppercase(),
            realm_name: config["realm_name"].as_str().unwrap().to_string(),
        }
    }

    fn parse_chat_config(config: &Yaml) -> BotChat {
        let greet_messages = config["greet_messages"].as_vec().unwrap();
        let agree = config["agree"].as_vec().unwrap();
        let disagree = config["disagree"].as_vec().unwrap();
        let follow_invite = config["follow_invite"].as_vec().unwrap();
        let stop = config["stop"].as_vec().unwrap();

        return BotChat {
            greet: greet_messages
                .into_iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            agree: agree
                .into_iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            disagree: disagree
                .into_iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            follow_invite: follow_invite
                .into_iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            stop: stop
                .into_iter()
                .map(|msg| msg.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        }
    }
}
