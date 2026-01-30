use std::sync::{Arc, Mutex};

mod auth_challenge;
mod set_time_speed;
mod request_characters;
mod set_weather;
mod motd;
mod select_character;
mod player_login;
mod account_data_times;
mod join_channels;
mod init_world_states;
mod verify_world;
mod addon_info;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Default)]
pub struct InitProcessor  {
    guid: Arc<Mutex<u64>>,
    is_logged_in: Arc<Mutex<bool>>,
}

impl Processor for InitProcessor {
    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;

        Ok(match opcode {
            Opcode::SMSG_AUTH_CHALLENGE => vec![
                Box::new(auth_challenge::Handler),
            ],
            Opcode::SMSG_AUTH_RESPONSE => vec![
                Box::new(account_data_times::Handler),
                Box::new(request_characters::Handler {
                    guid: self.guid.clone(),
                    is_logged_in: self.is_logged_in.clone(),
                }),
            ],
            Opcode::SMSG_LOGIN_VERIFY_WORLD => vec![
                Box::new(verify_world::Handler),
                Box::new(join_channels::Handler),
            ],
            Opcode::SMSG_MOTD => vec![
                Box::new(motd::Handler),
            ],
            Opcode::SMSG_LOGIN_SETTIMESPEED => vec![
                Box::new(set_time_speed::Handler),
            ],
            Opcode::SMSG_WEATHER => vec![
                Box::new(set_weather::Handler),
            ],
            Opcode::SMSG_INIT_WORLD_STATES => vec![
                Box::new(init_world_states::Handler),
            ],
            Opcode::SMSG_CHAR_ENUM => vec![
                Box::new(select_character::Handler {
                    guid: self.guid.clone(),
                }),
                Box::new(player_login::Handler {
                    guid: self.guid.clone(),
                    is_logged_in: self.is_logged_in.clone(),
                }),
            ],
            Opcode::SMSG_ADDON_INFO => vec![
                Box::new(addon_info::Handler),
            ],
            _ => vec![],
        })
    }
}