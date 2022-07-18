use std::collections::{VecDeque};
use std::io::Error;
use std::sync::{Arc};
use std::time::Duration;
use tokio::sync::{Mutex};
use tokio::net::TcpStream;
use tokio::task::{JoinHandle};
use tokio::time::{sleep};
use futures::future::join_all;

mod auth;
mod characters;
mod chat;
mod movement;
mod opcodes;
mod player;
mod realm;
mod spell;
mod trade;
mod types;
mod warden;

pub use characters::types::{Character};
pub use player::types::{Player, ObjectField, UnitField, PlayerField};
pub use realm::types::{Realm};
pub use warden::types::{WardenModuleInfo};

pub use movement::types::{MovementFlags, MovementFlagsExtra, SplineFlags, UnitMoveType};
pub use movement::parsers::position_parser::{PositionParser};
pub use movement::parsers::movement_parser::{MovementParser};
pub use movement::parsers::types::{MovementInfo};

use auth::AuthProcessor;
use characters::CharactersProcessor;
use chat::ChatProcessor;
use movement::MovementProcessor;
use player::PlayerProcessor;
use realm::RealmProcessor;
use spell::SpellProcessor;
use warden::WardenProcessor;

use movement::ai::AI as MovementAI;

// TODO: REMOVE THIS ! (need to think how better refactor this part)
use auth::login_challenge;
pub use crate::client::opcodes::Opcode;

use crate::client::types::ClientFlags;
use crate::crypto::warden_crypt::WardenCrypt;
use crate::data_storage::DataStorage;
use crate::network::session::Session;
use crate::network::stream::{Reader, Writer};

use crate::traits::Processor;
use crate::types::{
    AIManagerInput,
    HandlerInput,
    HandlerOutput,
    ProcessorFunction,
    ProcessorResult,
    State
};

// for each server need to play with this values
// for local server values can be much less then for external server
// for me it seems this values are related to ping, need to investigate this in future
const READ_TIMEOUT: u64 = 50;
const WRITE_TIMEOUT: u64 = 1;

pub struct Client {
    reader: Arc<Mutex<Option<Reader>>>,
    writer: Arc<Mutex<Option<Writer>>>,
    warden_crypt: Arc<Mutex<Option<WardenCrypt>>>,
    input_queue: Arc<Mutex<VecDeque<Vec<Vec<u8>>>>>,
    output_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    session: Arc<Mutex<Session>>,
    flags: Arc<Mutex<ClientFlags>>,
    data_storage: Arc<Mutex<DataStorage>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            reader: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
            warden_crypt: Arc::new(Mutex::new(None)),
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(VecDeque::new())),
            session: Arc::new(Mutex::new(Session::new())),
            flags: Arc::new(Mutex::new(ClientFlags::NONE)),
            data_storage: Arc::new(Mutex::new(DataStorage::new()))
        }
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<(), Error> {
        let stream = Self::connect_inner(host, port).await.unwrap();
        Self::set_stream_halves(stream, &self.reader, &self.writer).await;

        self.session.lock().await.set_config(host);

        Ok(())
    }

    async fn connect_inner(host: &str, port: u16) -> Result<TcpStream, Error> {
        let addr = format!("{}:{}", host, port);
        match TcpStream::connect(&addr).await {
            Ok(stream) => {
                println!("Connected to {}", addr);
                Ok(stream)
            },
            Err(err) => {
                panic!("Cannot connect: {:?}", err);
            },
        }
    }

    async fn set_stream_halves(
        stream: TcpStream,
        reader: &Arc<Mutex<Option<Reader>>>,
        writer: &Arc<Mutex<Option<Writer>>>
    ) {
        let (rx, tx) = stream.into_split();

        let mut reader = reader.lock().await;
        *reader = Some(Reader::new(rx));
        let mut writer = writer.lock().await;
        *writer = Some(Writer::new(tx));
    }

    pub async fn handle_connection(&mut self) {
        self.output_queue.lock().await.push_back(
            login_challenge(&self.session.lock().await.get_config().connection_data.username)
        );

        join_all(vec![
            self.handle_ai().await,
            self.handle_queue().await,
            self.handle_read().await,
            self.handle_write().await,
        ]).await;
    }

    async fn handle_ai(&mut self) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let data_storage = Arc::clone(&self.data_storage);
        let output_queue = Arc::clone(&self.output_queue);

        let mut movement_ai = MovementAI::new();

        tokio::spawn(async move {
            loop {
                movement_ai.manage(AIManagerInput {
                    session: Arc::clone(&session),
                    data_storage: Arc::clone(&data_storage),
                    output_queue: Arc::clone(&output_queue),
                }).await;

                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
            }
        })
    }

    async fn handle_queue(&mut self) -> JoinHandle<()> {
        let input_queue = Arc::clone(&self.input_queue);
        let output_queue = Arc::clone(&self.output_queue);
        let session = Arc::clone(&self.session);
        let reader = Arc::clone(&self.reader);
        let writer = Arc::clone(&self.writer);
        let warden_crypt = Arc::clone(&self.warden_crypt);
        let client_flags = Arc::clone(&self.flags);
        let data_storage = Arc::clone(&self.data_storage);

        tokio::spawn(async move {
            loop {
                let client_flags = &mut *client_flags.lock().await;

                let mut input_queue = input_queue.lock().await;
                let connected_to_realm = client_flags.contains(ClientFlags::IS_CONNECTED_TO_REALM);

                if let Some(packets) = input_queue.pop_front() {
                    for packet in packets {
                        let processors = match connected_to_realm {
                            true => Self::get_realm_processors(),
                            false => Self::get_login_processors(),
                        };

                        let session = &mut *session.lock().await;
                        let data_storage = &mut *data_storage.lock().await;
                        let output_list = processors
                            .iter()
                            .filter_map(|processor| {
                                let result: ProcessorResult = processor(HandlerInput {
                                    session,
                                    // packet: size + opcode + body, need to parse separately
                                    data: Some(&packet),
                                    data_storage,
                                });

                                result.ok()
                            })
                            .flatten()
                            .collect::<Vec<HandlerOutput>>();

                        for output in output_list {
                            match output {
                                HandlerOutput::Data((opcode, header, body)) => {
                                    let packet = match opcode {
                                        Opcode::CMSG_WARDEN_DATA => {
                                            let warden_crypt = &mut *warden_crypt.lock().await;
                                            [header, warden_crypt
                                                .as_mut().unwrap().encrypt(&body)
                                            ].concat()
                                        },
                                        _ => [header, body].concat(),
                                    };

                                    output_queue.lock().await.push_back(packet);
                                },
                                HandlerOutput::ConnectionRequest(host, port) => {
                                    let stream = Self::connect_inner(&host, port).await.unwrap();
                                    Self::set_stream_halves(stream, &reader, &writer).await;
                                },
                                HandlerOutput::UpdateState(state) => {
                                    match state {
                                        State::SetEncryption(session_key) => {
                                            *warden_crypt.lock().await = Some(
                                                WardenCrypt::new(&session_key)
                                            );

                                            if let Some(reader) = &mut *reader.lock().await {
                                                reader.init(
                                                    &session_key,
                                                    Arc::clone(&warden_crypt)
                                                );
                                            }

                                            if let Some(writer) = &mut *writer.lock().await {
                                                writer.init(&session_key);
                                            }
                                        },
                                        State::SetConnectedToRealm(is_authorized) => {
                                            client_flags.set(
                                                ClientFlags::IS_CONNECTED_TO_REALM,
                                                is_authorized
                                            );
                                        },
                                    }
                                },
                                HandlerOutput::Void => {},
                            };
                            sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
                        }
                    }
                } else {
                    sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
                }
            }
        })
    }

    async fn handle_write(&mut self) -> JoinHandle<()> {
        let output_queue = Arc::clone(&self.output_queue);
        let writer = Arc::clone(&self.writer);

        tokio::spawn(async move {
            loop {
                match output_queue.lock().await.pop_front() {
                    Some(packet) => {
                        if !packet.is_empty() {
                            match &mut *writer.lock().await {
                                Some(writer) => {
                                    writer.write(&packet).await;
                                },
                                None => {
                                    panic!("Not connected to TCP");
                                },
                            }
                        }
                    },
                    _ => {},
                };
                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
            }
        })
    }

    async fn handle_read(&mut self) -> JoinHandle<()> {
        let input_queue = Arc::clone(&self.input_queue);
        let reader = Arc::clone(&self.reader);

        tokio::spawn(async move {
            loop {
                match &mut *reader.lock().await {
                    Some(reader) => {
                        if let Some(packets) = reader.read().await.ok() {
                            input_queue.lock().await.push_back(packets);
                        }
                    },
                    None => {
                        panic!("Not connected to TCP");
                    },
                };

                sleep(Duration::from_millis(READ_TIMEOUT)).await;
            }
        })
    }

    fn get_login_processors<'a>() -> Vec<ProcessorFunction<'a>> {
        return vec![
            Box::new(AuthProcessor::process_input),
        ];
    }

    fn get_realm_processors<'a>() -> Vec<ProcessorFunction<'a>> {
        return vec![
            Box::new(CharactersProcessor::process_input),
            Box::new(ChatProcessor::process_input),
            Box::new(MovementProcessor::process_input),
            Box::new(PlayerProcessor::process_input),
            Box::new(RealmProcessor::process_input),
            Box::new(SpellProcessor::process_input),
            Box::new(WardenProcessor::process_input),
        ];
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use crate::Client;
    use crate::client::types::ClientFlags;
    use crate::network::session::types::{ActionFlags, StateFlags};

    const HOST: &str = "127.0.0.1";
    // https://users.rust-lang.org/t/async-tests-sometimes-fails/78451
    // port should be zero to avoid race condition (in case of running in parallel)
    // so OS will create connection with random port
    const PORT: u16 = 0;
    const WRONG_HOST: &str = "1.2.3.4";
    const PACKET: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    #[tokio::test]
    async fn test_client_create() {
        let client = Client::new();

        let reader = &mut *client.reader.lock().await;
        assert!(reader.is_none());

        let writer = &mut *client.writer.lock().await;
        assert!(writer.is_none());

        let warden_crypt = &mut *client.warden_crypt.lock().await;
        assert!(warden_crypt.is_none());

        let client_flags = &mut *client.flags.lock().await;
        assert_eq!(ClientFlags::NONE, *client_flags);

        let input_queue = &mut *client.input_queue.lock().await;
        assert!(input_queue.is_empty());

        let output_queue = &mut *client.output_queue.lock().await;
        assert!(output_queue.is_empty());

        let data_storage = &mut *client.data_storage.lock().await;
        assert!(data_storage.players_map.is_empty());

        let session = &mut *client.session.lock().await;
        assert!(session.session_key.is_none());
        assert!(session.me.is_none());
        assert!(session.warden_module_info.is_none());
        assert!(session.config.is_none());
        assert!(session.follow_target.is_none());
        assert!(session.server_id.is_none());
        assert!(session.party.is_empty());
        assert_eq!(ActionFlags::NONE, session.action_flags);
        assert_eq!(StateFlags::NONE, session.state_flags);

    }

    #[tokio::test]
    async fn test_client_connect() {
        let mut client = Client::new();
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(HOST, local_addr.port()).await.ok();

            let reader = &mut *client.reader.lock().await;
            assert!(reader.is_some());

            let writer = &mut *client.writer.lock().await;
            assert!(writer.is_some());
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_client_connect_with_wrong_data() {
        let mut client = Client::new();
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(WRONG_HOST, local_addr.port()).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_client_read_incoming_data() {
        let mut client = Client::new();
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(HOST, local_addr.port()).await.ok();

            if let Some((mut stream, _)) = listener.accept().await.ok() {
                stream.write(&PACKET).await.unwrap();
                stream.flush().await.unwrap();
                client.handle_read().await;

                loop {
                    if let Some(packet) = client.input_queue.lock().await.pop_front() {
                        assert_eq!(PACKET.to_vec(), packet[0]);
                        break;
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_client_write_outcoming_data() {
        let mut client = Client::new();
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(HOST, local_addr.port()).await.ok();
            client.output_queue.lock().await.push_back(PACKET.to_vec());

            if let Some((stream, _)) = listener.accept().await.ok() {
                let buffer_size = PACKET.to_vec().len();
                let mut buffer = Vec::with_capacity(buffer_size);

                client.handle_write().await;
                stream.take(buffer_size as u64).read_to_end(&mut buffer).await.unwrap();

                assert_eq!(PACKET.to_vec(), buffer);
            }
        }
    }
}