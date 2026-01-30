use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use regex::Regex;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::{login, realm};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
pub struct Incoming {
    skip: [u8; 6],
    realms_count: u16,
    #[br(count = realms_count)]
    realms: Vec<Realm>,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut outputs = vec![];
        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        let autoselect = config.autoselect.ok_or_else(|| anyhow::anyhow!("Missing [autoselect]"))?;
        let Incoming { realms, .. } = Incoming::unpack(packet)?;

        if realms.is_empty() {
            return Ok(vec![
                HandlerOutput::Messages(vec![
                    Message {
                        msg_type: MsgType::Error,
                        text: "Realmlist is empty".to_string(),
                    }
                ])
            ])
        }

        let items = Arc::new(realms.iter().map(|r| r.to_string()).collect());

        if !autoselect.realm_name.is_empty() {
            let re = Regex::new(&autoselect.realm_name)?;
            if let Some(realm) = realms.into_iter().find(|item| re.is_match(&item.name[..])) {
                let Realm { name, address, server_id, .. } = realm;
                let target: ServerLabel = realm::PLUGIN_LABEL;

                outputs.extend(vec![
                    HandlerOutput::Messages(vec![
                        Message {
                            msg_type: MsgType::Info,
                            text: format!("Connecting to \"{name}\"-realm"),
                        }
                    ]),
                    HandlerOutput::Requests(vec![
                        Request::Connect(
                            target,
                            address.to_string(),
                        ),
                        Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                            ctx.insert(ServerId(server_id));
                        }))),
                        Request::Drop(login::PLUGIN_LABEL),
                    ]),
                ]);
            }
        } else {
            outputs.push(HandlerOutput::Requests(vec![
                Request::InitChoice(ChoiceItems(items), Some(Box::new(move |ids: Vec<usize>| {
                    let selected = &realms[ids[0]];
                    let name: String = selected.name.to_string();
                    let address: String = selected.address.to_string();
                    let server_id: u8 = selected.server_id;
                    let target: ServerLabel = realm::PLUGIN_LABEL;

                    Ok(vec![
                        HandlerOutput::Messages(vec![
                            Message {
                                msg_type: MsgType::Info,
                                text: format!("Connecting to \"{name}\"-realm"),
                            }
                        ]),
                        HandlerOutput::Requests(vec![
                            Request::Connect(
                                target,
                                address,
                            ),
                            Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                                ctx.insert(ServerId(server_id));
                            }))),
                            Request::Drop(login::PLUGIN_LABEL),
                        ]),
                    ])
                })))
            ]));
        }

        Ok(outputs)
    }
}

pub struct ServerId(pub u8);

#[derive(BinRead, Serialize, FieldsMetadata, Debug, Default)]
#[br(little)]
pub struct Realm {
    pub icon: u8,
    pub lock: u8,
    pub flags: u8,
    pub name: NullTerminated<String>,
    pub address: NullTerminated<String>,
    pub population: f32,
    pub characters_amount: u8,
    pub timezone: u8,
    pub server_id: u8,
}

impl Display for Realm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]({}) #id: {}, #pop: {}, #tz: {}, #chars: {}",
            self.name,
            self.address,
            self.server_id,
            self.population,
            self.timezone,
            self.characters_amount,
        )
    }
}