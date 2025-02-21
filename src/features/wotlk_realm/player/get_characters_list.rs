use async_trait::async_trait;
use regex::Regex;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::position::Point3D;
use tentacli_traits::types::shared::Object;

use crate::features::wotlk_realm::globals::CharacterEnumOutgoing;
use crate::features::wotlk_realm::player::packet::CharCreateOutgoing;
use crate::features::wotlk_realm::player::traits::CharacterCreateToolkit;

#[derive(WorldPacket, Serialize, Debug)]
struct Incoming {
    characters_count: u8,
    #[depends_on(characters_count)]
    characters: Vec<Character>,
}

#[derive(Segment, Serialize, Debug, Clone, Default)]
struct Character {
    guid: u64,
    name: String,
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
    location: Point3D,
    guild_id: u32,
    flags: u32,
    customize_flags: u32,
    first_login: bool,
    pet_info: PetInfo,
    equipment: [EquippedItem; 23],
}

#[derive(Segment, Serialize, Debug, Clone, Default)]
struct PetInfo {
    display_id: u32,
    level: u32,
    family: u32,
}

#[derive(Segment, Serialize, Debug, Clone, Default)]
struct EquippedItem {
    display_id: u32,
    inventory_type: u8,
    aura_id: u32,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { characters, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let (enable_auto_create, name_pattern) = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            let enable_auto_create = config.common.auto_create_character_for_new_account;
            let name_pattern = config.connection_data.autoselect_character_name.to_string();

            (enable_auto_create, name_pattern)
        };

        let player_objects = characters.into_iter().map(|c| Object {
            guid: c.guid,
            name: c.name,
            ..Default::default()
        }).collect::<Vec<Object>>();

        match player_objects.is_empty() {
            true if enable_auto_create => {
                let random_name = Self::generate_random_name();
                response.push(HandlerOutput::ResponseMessage(
                    format!("Creating character with name \"{}\"", random_name),
                    None,
                ));

                response.push(HandlerOutput::Data(CharCreateOutgoing {
                    name: format!("{}\0", random_name),
                    race: Self::get_random_race(),
                    class: Self::get_random_class(),
                    gender: Self::get_random_gender(),
                    skin: 0,
                    face: 0,
                    hair_style: 0,
                    hair_color: 0,
                    facial_hair: 0,
                    outfit_id: 0,
                }.unpack_with_client_opcode(Opcode::CMSG_CHAR_CREATE)?));

                response.push(
                    HandlerOutput::Data(
                        CharacterEnumOutgoing::default()
                            .unpack_with_client_opcode(Opcode::CMSG_CHAR_ENUM)?
                    )
                );
            }
            false if name_pattern.is_empty() => {
                response.push(HandlerOutput::TransferCharactersList(player_objects));
                response.push(HandlerOutput::Freeze);
            }
            false => {
                let re = Regex::new(&name_pattern)?;
                let character = player_objects.into_iter().find(|item| re.is_match(&item.name[..]));

                if let Some(character) = character {
                    response.push(HandlerOutput::DebugMessage(
                        format!("Selected \"{}\" Character", character.name),
                        None,
                    ));

                    response.push(HandlerOutput::SelectCharacter(character.guid));

                    input.session.lock().await.my_guid = Some(character.guid);
                }
            }
            _ => {}
        }

        Ok(response)
    }
}

impl CharacterCreateToolkit for Handler {}