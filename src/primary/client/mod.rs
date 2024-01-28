use std::env;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex as SyncMutex};
use std::time::Duration;
use tokio::sync::{Mutex, Notify};
use async_broadcast::{broadcast, Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::net::TcpStream;
use tokio::task::{JoinHandle};
use futures::future::{join_all};
use tokio::time::sleep;
use anyhow::{Result as AnyResult};
use std::str::FromStr;

mod auth;
mod characters;
mod chat;
mod movement;
mod opcodes;
mod player;
mod realm;
mod spell;
mod trade;
pub mod types;
mod warden;

pub use characters::types::{Character};
pub use movement::types::{MovementFlags, MovementFlagsExtra, SplineFlags, UnitMoveType};
pub use player::types::{Player, ObjectField, UnitField, PlayerField, FieldType, FieldValue};
pub use realm::types::{Realm};
pub use spell::types::{Spell, CooldownInfo};
pub use warden::types::{WardenModuleInfo};

use auth::AuthProcessor;
use characters::CharactersProcessor;
use chat::ChatProcessor;
use movement::MovementProcessor;
use player::PlayerProcessor;
use realm::RealmProcessor;
use spell::SpellProcessor;
use warden::WardenProcessor;

// TODO: REMOVE THIS ! (need to think how better refactor this part)
use auth::login_challenge;

pub use crate::primary::client::opcodes::Opcode;
use crate::primary::client::types::ClientFlags;
use crate::primary::crypto::encryptor::OUTCOMING_HEADER_LENGTH;
use crate::primary::crypto::warden_crypt::WardenCrypt;
use crate::primary::shared::storage::DataStorage;
use crate::primary::shared::session::Session;
use crate::primary::network::stream::{Reader, Writer};
use crate::primary::traits::Feature;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, HandlerOutput, PacketOutcome, ProcessorFunction, ProcessorResult, Signal};

pub struct Client {
    _reader: Arc<Mutex<Option<Reader>>>,
    _writer: Arc<Mutex<Option<Writer>>>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _flags: Arc<SyncMutex<ClientFlags>>,

    session: Arc<SyncMutex<Session>>,
    data_storage: Arc<SyncMutex<DataStorage>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            _reader: Arc::new(Mutex::new(None)),
            _writer: Arc::new(Mutex::new(None)),
            _warden_crypt: Arc::new(SyncMutex::new(None)),
            _flags: Arc::new(SyncMutex::new(ClientFlags::NONE)),

            session: Arc::new(SyncMutex::new(Session::new())),
            data_storage: Arc::new(SyncMutex::new(DataStorage::new())),
        }
    }

    async fn connect_inner(host: &str, port: u16) -> Result<TcpStream, Error> {
        let addr = format!("{}:{}", host, port);
        match TcpStream::connect(&addr).await {
            Ok(stream) => Ok(stream),
            Err(err) => Err(err),
        }
    }

    async fn set_stream_halves(
        stream: TcpStream,
        reader: Arc<Mutex<Option<Reader>>>,
        writer: Arc<Mutex<Option<Writer>>>,
        session_key: Option<Vec<u8>>,
        warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    ) {
        let (rx, tx) = stream.into_split();

        if session_key.is_none() {
            *reader.lock().await = Some(Reader::new(rx));
            *writer.lock().await = Some(Writer::new(tx));
        } else {
            let session_key = session_key.unwrap();
            *warden_crypt.lock().unwrap() = Some(WardenCrypt::new(&session_key));

            let mut _reader = Reader::new(rx);
            _reader.init(&session_key, Arc::clone(&warden_crypt));
            *reader.lock().await = Some(_reader);

            let mut _writer = Writer::new(tx);
            _writer.init(&session_key);
            *writer.lock().await = Some(_writer);
        }
    }

    pub async fn run(&mut self) -> AnyResult<()> {
        const BUFFER_SIZE: usize = 50;

        let notify = Arc::new(Notify::new());

        let (signal_sender, signal_receiver) = mpsc::channel::<Signal>(1);
        let (input_sender, input_receiver) = mpsc::channel::<Vec<u8>>(BUFFER_SIZE);
        let (output_sender, output_receiver) = mpsc::channel::<PacketOutcome>(BUFFER_SIZE);
        let (query_sender, query_receiver) = broadcast::<HandlerOutput>(BUFFER_SIZE);

        let host = env::var("CURRENT_HOST").expect("CURRENT_HOST must be set");
        let port = env::var("CURRENT_PORT").expect("CURRENT_PORT must be set");

        match Self::connect_inner(&host, u16::from_str(&port)?).await {
            Ok(stream) => {
                Self::set_stream_halves(
                    stream,
                    Arc::clone(&self._reader),
                    Arc::clone(&self._writer),
                    None,
                    Arc::clone(&self._warden_crypt),
                ).await;

                match self.session.lock().unwrap().set_config(&host) {
                    Ok(_) => {},
                    Err(err) => {
                        query_sender.broadcast(HandlerOutput::ErrorMessage(err.to_string(), None)).await.unwrap();
                    }
                }

                query_sender.broadcast(
                    HandlerOutput::SuccessMessage(
                        format!("Connected to {}:{}", host, port),
                        None
                    )
                ).await.unwrap();

                Ok(())
            },
            Err(err) => {
                query_sender.broadcast(HandlerOutput::ErrorMessage(format!("Cannot connect: {}", err), None)).await.unwrap();

                Err(err)
            },
        }?;

        let mut features: Vec<Box<dyn Feature>> = vec![];
        cfg_if! {
            if #[cfg(feature = "ui")] {
                use crate::features::ui::UI;

                let ui = UI::new(query_sender.clone(), query_receiver.clone());
                features.push(Box::new(ui));
            } else if #[cfg(feature = "console")] {
                use crate::features::console::Console;

                let console = Console::new(query_sender.clone(), query_receiver.clone());
                features.push(Box::new(console));
            }
        }

        let account = {
            let guard = self.session.lock().unwrap();
            let config = guard.get_config()?;
            config.connection_data.account.to_string()
        };

        {
            query_sender.broadcast(
                HandlerOutput::RequestMessage(format!("LOGIN_CHALLENGE as {}", &account), None)
            ).await?;
        }

        output_sender.send(login_challenge(&account)?).await?;

        let mut all_tasks = vec![
            self.handle_read(input_sender.clone(), signal_receiver, query_sender.clone()),
            self.handle_write(output_receiver, query_sender.clone()),
            self.handle_packet(input_receiver, query_sender.clone(), notify.clone()),
            self.handle_output(
                signal_sender.clone(), output_sender.clone(), query_sender.clone(), query_receiver, notify.clone()
            ),
        ];

        let features_tasks: Vec<JoinHandle<()>> =
            features.into_iter().map(|mut feature| feature.get_tasks()).flatten().collect();

        all_tasks.extend(features_tasks);

        join_all(all_tasks).await;

        Ok(())
    }

    fn handle_packet(
        &mut self,
        mut input_receiver: Receiver<Vec<u8>>,
        query_sender: BroadcastSender<HandlerOutput>,
        notify: Arc<Notify>,
    ) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let client_flags = Arc::clone(&self._flags);
        let data_storage = Arc::clone(&self.data_storage);

        tokio::spawn(async move {
            loop {
                if let Some(packet) = input_receiver.recv().await {
                    let processors = {
                        let connected_to_realm = {
                            client_flags.lock().unwrap().contains(ClientFlags::IS_CONNECTED_TO_REALM)
                        };

                        match connected_to_realm {
                            true => Self::get_realm_processors(),
                            false => Self::get_login_processors(),
                        }
                    };

                    let mut input = HandlerInput {
                        session: Arc::clone(&session),
                        data: Some(packet),
                        data_storage: Arc::clone(&data_storage),
                        opcode: None,
                    };

                    let handler_list = processors
                        .iter()
                        .flat_map(|processor| processor(&mut input))
                        .collect::<ProcessorResult>();

                    for mut handler in handler_list {
                        let response = handler.handle(&mut input).await;
                        match response {
                            Ok(outputs) => {
                                for output in outputs {
                                    match output {
                                        HandlerOutput::Freeze => {
                                            notify.notified().await;
                                        },
                                        _ => {
                                            query_sender.broadcast(output).await.unwrap();
                                        },
                                    }
                                }
                            },
                            Err(err) => {
                                query_sender.broadcast(HandlerOutput::ErrorMessage(err.to_string(), None)).await.unwrap();
                            },
                        };
                    }
                }
            }
        })
    }

    fn handle_output(
        &mut self,
        signal_sender: Sender<Signal>,
        output_sender: Sender<PacketOutcome>,
        query_sender: BroadcastSender<HandlerOutput>,
        mut query_receiver: BroadcastReceiver<HandlerOutput>,
        notify: Arc<Notify>,
    ) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let reader = Arc::clone(&self._reader);
        let writer = Arc::clone(&self._writer);
        let warden_crypt = Arc::clone(&self._warden_crypt);
        let client_flags = Arc::clone(&self._flags);

        tokio::spawn(async move {
            loop {
                let result = query_receiver.recv().await;
                match result {
                    Ok(output) => {
                        match output {
                            HandlerOutput::Data((opcode, packet, json)) => {
                                let packet = match opcode {
                                    Opcode::CMSG_WARDEN_DATA => {
                                        let header = &packet[..OUTCOMING_HEADER_LENGTH];
                                        let body = warden_crypt.lock()
                                            .unwrap().as_mut()
                                            .unwrap().encrypt(&packet[OUTCOMING_HEADER_LENGTH..]);

                                        [header.to_vec(), body.to_vec()].concat()
                                    },
                                    _ => packet,
                                };

                                output_sender.send((opcode, packet, json)).await.unwrap();
                            },
                            HandlerOutput::ConnectionRequest(host, port) => {
                                match Self::connect_inner(&host, port).await {
                                    Ok(stream) => {
                                        signal_sender.send(Signal::Reconnect).await.unwrap();

                                        let session_key = {
                                            let guard = session.lock().unwrap();
                                            guard.session_key.clone()
                                        };

                                        Self::set_stream_halves(
                                            stream,
                                            Arc::clone(&reader),
                                            Arc::clone(&writer),
                                            session_key.clone(),
                                            Arc::clone(&warden_crypt),
                                        ).await;

                                        query_sender.broadcast(
                                            HandlerOutput::SuccessMessage(
                                                format!("Connected to {}:{}", host, port),
                                                None
                                            )
                                        ).await.unwrap();

                                        client_flags.lock().unwrap().set(
                                            ClientFlags::IS_CONNECTED_TO_REALM,
                                            true,
                                        );
                                    },
                                    Err(err) => {
                                        query_sender.broadcast(
                                            HandlerOutput::ErrorMessage(err.to_string(), None)
                                        ).await.unwrap();
                                    }
                                }
                            },
                            HandlerOutput::Drop => {
                                break;
                            },
                            HandlerOutput::SelectRealm(realm) => {
                                session.lock().unwrap().selected_realm = Some(realm);
                                notify.notify_one();
                            },
                            HandlerOutput::SelectCharacter(character) => {
                                session.lock().unwrap().me = Some(Player::from(character));
                                notify.notify_one();
                            },
                            _ => {},
                        };
                    },
                    Err(err) => {
                        query_sender.broadcast(HandlerOutput::ErrorMessage(err.to_string(), None)).await.unwrap();
                    },
                };
            }
        })
    }

    fn handle_write(
        &mut self,
        mut output_receiver: Receiver<PacketOutcome>,
        query_sender: BroadcastSender<HandlerOutput>,
    ) -> JoinHandle<()> {
        let writer = Arc::clone(&self._writer);

        tokio::spawn(async move {
            loop {
                if let Some((opcode, packet, json)) = output_receiver.recv().await {
                    if !packet.is_empty() {
                        let result = Self::write_packet(&writer, packet).await;

                        match result {
                            Ok(bytes_sent) => {
                                let message = format!(
                                    "{}: {} bytes sent",
                                    Opcode::get_client_opcode_name(opcode),
                                    bytes_sent,
                                );

                                query_sender.broadcast(HandlerOutput::RequestMessage(message, Some(json))).await.unwrap();
                            },
                            Err(err) => {
                                query_sender.broadcast(HandlerOutput::ErrorMessage(err.to_string(), None)).await.unwrap();
                            }
                        }
                    }
                }
            }
        })
    }

    fn handle_read(
        &mut self,
        input_sender: Sender<Vec<u8>>,
        mut signal_receiver: Receiver<Signal>,
        query_sender: BroadcastSender<HandlerOutput>,
    ) -> JoinHandle<()> {
        let reader = Arc::clone(&self._reader);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = signal_receiver.recv() => {},
                    result = Self::read_packet(&reader) => {
                        match result {
                            Ok(packets) => {
                                input_sender.send(packets).await.unwrap();
                            },
                            Err(err) => {
                                query_sender.broadcast(HandlerOutput::ErrorMessage(err.to_string(), None)).await.unwrap();
                                sleep(Duration::from_secs(1)).await;
                            }
                        }
                    },
                };
            }
        })
    }

    async fn read_packet(reader: &Arc<Mutex<Option<Reader>>>) -> Result<Vec<u8>, Error> {
        let mut error = Error::new(ErrorKind::NotFound, "Not connected to TCP");

        if let Some(reader) = &mut *reader.lock().await {
            let result = reader.read().await;
            match result {
                Ok(packet) => {
                    if !packet.is_empty() {
                        return Ok(packet);
                    }
                }
                Err(err) => {
                    error = err;
                },
            }
        }

        Err(error)
    }

    async fn write_packet(
        writer: &Arc<Mutex<Option<Writer>>>,
        packet: Vec<u8>
    ) -> Result<usize, Error> {
        let mut error = Error::new(ErrorKind::NotFound, "Not connected to TCP");

        if let Some(writer) = &mut *writer.lock().await {
            match writer.write(&packet).await {
                Ok(bytes_sent) => {
                    return Ok(bytes_sent);
                },
                Err(err) => {
                    error = err;
                }
            };
        }

        Err(error)
    }

    fn get_login_processors() -> Vec<ProcessorFunction> {
        vec![
            Box::new(AuthProcessor::process_input),
        ]
    }

    fn get_realm_processors() -> Vec<ProcessorFunction> {
        vec![
            Box::new(CharactersProcessor::process_input),
            Box::new(ChatProcessor::process_input),
            Box::new(MovementProcessor::process_input),
            Box::new(PlayerProcessor::process_input),
            Box::new(RealmProcessor::process_input),
            Box::new(SpellProcessor::process_input),
            Box::new(WardenProcessor::process_input),
        ]
    }
}

#[cfg(test)]
impl Client {
    pub async fn connect(&mut self, host: &str, port: u16) -> Result<(), Error> {
        match Self::connect_inner(&host, port).await {
            Ok(stream) => {
                Self::set_stream_halves(
                    stream,
                    Arc::clone(&self._reader),
                    Arc::clone(&self._writer),
                    None,
                    Arc::clone(&self._warden_crypt),
                ).await;

                match self.session.lock().unwrap().set_config(&host) {
                    Ok(_) => {},
                    Err(_) => {}
                }

                Ok(())
            },
            Err(err) => {
                Err(err)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use async_broadcast::broadcast;
    use futures::FutureExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;

    use crate::Client;
    use crate::primary::client::types::ClientFlags;
    use crate::primary::shared::session::types::{ActionFlags, StateFlags};
    use crate::primary::types::{HandlerOutput, PacketOutcome, Signal};

    const HOST: &str = "127.0.0.1";
    // https://users.rust-lang.org/t/async-tests-sometimes-fails/78451
    // port should be zero to avoid race condition (in case of running in parallel)
    // so OS will create connection with random port
    const PORT: u16 = 0;
    const PACKET: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    #[tokio::test]
    async fn test_client_create() {
        let client = Client::new();

        let reader = &mut *client._reader.lock().await;
        assert!(reader.is_none());

        let writer = &mut *client._writer.lock().await;
        assert!(writer.is_none());

        let warden_crypt = &mut *client._warden_crypt.lock().unwrap();
        assert!(warden_crypt.is_none());

        let client_flags = &mut *client._flags.lock().unwrap();
        assert_eq!(ClientFlags::NONE, *client_flags);

        let data_storage = &mut *client.data_storage.lock().unwrap();
        assert!(data_storage.players_map.is_empty());

        let session = &mut *client.session.lock().unwrap();
        assert!(session.session_key.is_none());
        assert!(session.me.is_none());
        assert!(session.warden_module_info.is_none());
        assert!(session.config.is_none());
        assert!(session.follow_target.is_none());
        assert!(session.selected_realm.is_none());
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

            let reader = &mut *client._reader.lock().await;
            assert!(reader.is_some());

            let writer = &mut *client._writer.lock().await;
            assert!(writer.is_some());
        }
    }

    #[tokio::test]
    async fn test_client_read_incoming_data() {
        let mut client = Client::new();
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(HOST, local_addr.port()).await.ok();

            let (_, signal_receiver) = mpsc::channel::<Signal>(1);
            let (input_sender, mut input_receiver) = mpsc::channel::<Vec<u8>>(1);
            let (query_sender, _) = broadcast::<HandlerOutput>(1);

            if let Some((mut stream, _)) = listener.accept().await.ok() {
                stream.write(&PACKET).await.unwrap();
                stream.flush().await.unwrap();

                let read_task = client.handle_read(input_sender, signal_receiver, query_sender);

                let test_task = tokio::spawn(async move {
                    loop {
                        if let Some(packet) = input_receiver.recv().await {
                            assert_eq!(PACKET.to_vec(), packet);
                            break;
                        }
                    }
                });

                tokio::select! {
                    _ = read_task.fuse() => {},
                    _ = test_task.fuse() => {},
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

            let (output_sender, output_receiver) = mpsc::channel::<PacketOutcome>(1);
            let (query_sender, _) = broadcast::<HandlerOutput>(1);

            output_sender.send((0, PACKET.to_vec(), String::new())).await.unwrap();

            if let Some((stream, _)) = listener.accept().await.ok() {
                let buffer_size = PACKET.to_vec().len();
                let mut buffer = Vec::with_capacity(buffer_size);

                client.handle_write(output_receiver, query_sender);
                stream.take(buffer_size as u64).read_to_end(&mut buffer).await.unwrap();

                assert_eq!(PACKET.to_vec(), buffer);
            }
        }
    }
}