use async_trait::async_trait;
use binrw::{BinRead, BinWrite};
use rand::distr::Alphanumeric;
use rand::prelude::IndexedRandom;
use rand::{Rng, rng};
use regex::Regex;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::opcodes::Opcode;
use crate::plugins::wow::wotlk::realm::object::types::movement::Point3D;
use crate::plugins::wow::wotlk::realm::object::types::unit::{Class, Gender, Race};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    characters_count: u8,
    #[br(count = characters_count)]
    characters: Vec<Character>,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_CHAR_CREATE)]
#[opcode(U32(Opcode::CMSG_CHAR_CREATE))]
#[bw(little)]
struct CharCreate {
    #[bw(write_with = crate::client::packet::helpers::null_terminated)]
    name: NullTerminated<String>,
    race: u8,
    class: u8,
    gender: u8,
    skin: u8,
    face: u8,
    hair_style: u8,
    hair_color: u8,
    facial_hair: u8,
    outfit_id: u8,
}

pub struct Handler {
    pub guid: Arc<Mutex<u64>>,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut outputs = vec![];
        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        let autoselect = config
            .autoselect
            .ok_or_else(|| anyhow::anyhow!("Missing [autoselect]"))?;
        let common = config
            .common
            .ok_or_else(|| anyhow::anyhow!("Missing [common]"))?;

        let Incoming { characters, .. } = Incoming::unpack(packet)?;

        if characters.is_empty() {
            if common.create_character_if_empty {
                outputs.push(HandlerOutput::Packets(vec![
                    CharCreate {
                        name: generate_random_name().into(),
                        race: generate_random_race(),
                        class: generate_random_class(),
                        gender: generate_random_gender(),
                        skin: 0,
                        face: 0,
                        hair_style: 0,
                        hair_color: 0,
                        facial_hair: 0,
                        outfit_id: 0,
                    }
                    .pack()?,
                ]));
            } else {
                outputs.push(HandlerOutput::Messages(vec![Message {
                    msg_type: MsgType::Error,
                    text: "Character list is empty, \
                        you should create new character to proceed."
                        .to_string(),
                }]))
            }
        } else if autoselect.character_name.is_empty() {
            let items = Arc::new(characters.iter().map(|c| c.to_string()).collect());
            let arc_guid = self.guid.clone();

            outputs.push(HandlerOutput::Requests(vec![Request::InitChoice(
                ChoiceItems(items),
                Some(Box::new(move |ids: Vec<usize>| {
                    let Character { name, guid, .. } = &characters[ids[0]];
                    *arc_guid.lock().unwrap() = *guid;

                    Ok(vec![HandlerOutput::Messages(vec![Message {
                        msg_type: MsgType::Info,
                        text: format!("Selected \"{name}\" character"),
                    }])])
                })),
            )]))
        } else {
            let re = Regex::new(&autoselect.character_name)?;
            if let Some(character) = characters
                .into_iter()
                .find(|item| re.is_match(item.name.as_ref()))
            {
                let Character { name, guid, .. } = character;
                *self.guid.lock().unwrap() = guid;
                outputs.push(HandlerOutput::Messages(vec![Message {
                    msg_type: MsgType::Info,
                    text: format!("Selected \"{name}\" character"),
                }]));
            }
        }

        Ok(outputs)
    }
}

fn generate_random_name() -> String {
    let mut rng = rng();
    let random_length = rng.random_range(9..=11);

    let string: String = rng
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic())
        .take(random_length)
        .map(|c| c as char)
        .collect();

    let first_letter = string.chars().next().unwrap();

    format!("{}{}", first_letter.to_uppercase(), string.to_lowercase())
}

fn generate_random_race() -> u8 {
    let races = &[
        Race::Human,
        Race::Orc,
        Race::Dwarf,
        Race::NightElf,
        Race::Undead,
        Race::Troll,
    ];

    let mut rng = rng();
    (*races.choose(&mut rng).unwrap()).into()
}

fn generate_random_class() -> u8 {
    let races = &[Class::Warrior, Class::Rogue];

    let mut rng = rng();
    (*races.choose(&mut rng).unwrap()).into()
}

fn generate_random_gender() -> u8 {
    let races = &[Gender::Male, Gender::Female];

    let mut rng = rng();
    (*races.choose(&mut rng).unwrap()).into()
}

#[derive(BinRead, FieldsMetadata, Serialize)]
struct Character {
    guid: u64,
    name: NullTerminated<String>,
    race: u8,
    class: u8,
    gender: u8,
    skin: u8,
    face: u8,
    hair_style: u8,
    hair_color: u8,
    facial_hair: u8,
    level: u8,
    zone_id: u32,
    map_id: u32,
    position: Point3D,
    guild_id: u32,
    flags: u32,
    customize_flags: u32,
    #[br(map = |x: u8| x != 0)]
    first_login: bool,
    pet_info: PetInfo,
    equipment: [EquippedItem; 23],
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} lvl]: {}, {}, {}, (zone={}/map={})",
            self.level, self.name, self.race, self.class, self.zone_id, self.map_id
        )
    }
}

#[derive(BinRead, FieldsMetadata, Serialize)]
struct PetInfo {
    display_id: u32,
    level: u32,
    family: u32,
}

#[derive(BinRead, FieldsMetadata, Serialize)]
struct EquippedItem {
    display_id: u32,
    inventory_type: u8,
    aura_id: u32,
}
