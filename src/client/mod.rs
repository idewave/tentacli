use std::collections::{VecDeque};
use std::io::{Error, Stdout};
use std::sync::{Arc, Mutex as SyncMutex};
use std::time::Duration;
use tokio::sync::{Mutex};
use tokio::net::TcpStream;
use tokio::task::{JoinHandle};
use tokio::time::{sleep};
use futures::future::join_all;
use tui::backend::CrosstermBackend;

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
use crate::ipc::pipe::{IncomeMessagePipe, OutcomeMessagePipe};
use crate::ipc::storage::DataStorage;
use crate::ipc::session::Session;
use crate::network::stream::{Reader, Writer};
use crate::types::traits::Processor;
use crate::types::{
    AIManagerInput,
    HandlerInput,
    HandlerOutput,
    HandlerFunction,
    ProcessorFunction,
    ProcessorResult,
    State
};
use crate::UI;
use crate::ui::types::{UIOutputOptions, UIRenderOptions};
use crate::ui::{UIInput, UIOutput};

// for each server need to play with this values
// for local server values can be much less then for external server
// for me it seems this values are related to ping, need to investigate this in future
const READ_TIMEOUT: u64 = 50;
const WRITE_TIMEOUT: u64 = 1;

pub struct Client {
    _reader: Arc<Mutex<Option<Reader>>>,
    _writer: Arc<Mutex<Option<Writer>>>,
    _warden_crypt: Arc<Mutex<Option<WardenCrypt>>>,
    _input_queue: Arc<Mutex<VecDeque<Vec<Vec<u8>>>>>,
    _output_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    _income_message_pipe: Arc<Mutex<IncomeMessagePipe>>,
    _outcome_message_pipe: Arc<Mutex<OutcomeMessagePipe>>,

    session: Arc<SyncMutex<Session>>,
    data_storage: Arc<SyncMutex<DataStorage>>,
    client_flags: Arc<Mutex<ClientFlags>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            _reader: Arc::new(Mutex::new(None)),
            _writer: Arc::new(Mutex::new(None)),
            _warden_crypt: Arc::new(Mutex::new(None)),
            _input_queue: Arc::new(Mutex::new(VecDeque::new())),
            _output_queue: Arc::new(Mutex::new(VecDeque::new())),
            _income_message_pipe: Arc::new(Mutex::new(IncomeMessagePipe::new())),
            _outcome_message_pipe: Arc::new(Mutex::new(OutcomeMessagePipe::new())),

            session: Arc::new(SyncMutex::new(Session::new())),
            data_storage: Arc::new(SyncMutex::new(DataStorage::new())),
            client_flags: Arc::new(Mutex::new(ClientFlags::NONE)),
        }
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<(), Error> {
        return match Self::connect_inner(host, port).await {
            Ok(stream) => {
                Self::set_stream_halves(stream, &self._reader, &self._writer).await;
                self.session.lock().unwrap().set_config(host);
                self._income_message_pipe.lock().await.message_income.send_success_message(
                    format!("Connected to {}:{}", host, port)
                );

                Ok(())
            },
            Err(err) => {
                self._income_message_pipe.lock().await.message_income.send_error_message(
                    format!("Cannot connect: {}", err.to_string())
                );

                Err(err)
            },
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
        // TODO: remove this part after in favor of manual packet sending
        match self.session.lock().unwrap().get_config() {
            Some(config) => {
                let username = &config.connection_data.username;
                self._income_message_pipe.lock().await.message_income.send_client_message(
                    format!("LOGIN_CHALLENGE as {}", username)
                );

                self._output_queue.lock().await.push_back(login_challenge(username));
            },
            None => {},
        }

        join_all(vec![
            self.handle_ui_render().await,
            self.handle_ui_input().await,
            self.handle_ui_output().await,
            self.handle_ai().await,
            self.handle_queue().await,
            self.handle_read().await,
            self.handle_write().await,
        ]).await;
    }

    async fn handle_ui_input(&mut self) -> JoinHandle<()> {
        let key_event_income = self._income_message_pipe.lock().await.key_event_income.clone();

        tokio::spawn(async move {
            let mut ui_input = UIInput::new(key_event_income);

            loop {
                ui_input.handle().await;
            }
        })
    }

    async fn handle_ui_render(&mut self) -> JoinHandle<()> {
        let income_message_pipe = Arc::clone(&self._income_message_pipe);
        let dialog_outcome = self._outcome_message_pipe.lock().await.dialog_outcome.clone();

        tokio::spawn(async move {
            let mut ui = UI::new(
                CrosstermBackend::new(std::io::stdout()),
                UIOutputOptions {
                    dialog_outcome,
                },
            );

            loop {
                ui.render(UIRenderOptions {
                    message: income_message_pipe.lock().await.recv(),
                });

                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
            }
        })
    }

    async fn handle_ui_output(&mut self) -> JoinHandle<()> {
        let outcome_message_pipe = Arc::clone(&self._outcome_message_pipe);
        let session = Arc::clone(&self.session);
        let client_flags = Arc::clone(&self.client_flags);

        tokio::spawn(async move {
            loop {
                if let Ok(message) = outcome_message_pipe.lock().await.recv() {
                    let client_flags = &mut *client_flags.lock().await;

                    let mut ui_output = UIOutput::new(Arc::clone(&session), client_flags);
                    ui_output.handle(message);
                }
                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
            }
        })
    }

    async fn handle_ai(&mut self) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let data_storage = Arc::clone(&self.data_storage);
        let output_queue = Arc::clone(&self._output_queue);

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
        let input_queue = Arc::clone(&self._input_queue);
        let output_queue = Arc::clone(&self._output_queue);
        let session = Arc::clone(&self.session);
        let reader = Arc::clone(&self._reader);
        let writer = Arc::clone(&self._writer);
        let warden_crypt = Arc::clone(&self._warden_crypt);
        let client_flags = Arc::clone(&self.client_flags);
        let data_storage = Arc::clone(&self.data_storage);
        let mut message_income = self._income_message_pipe.lock().await.message_income.clone();
        let dialog_income = self._income_message_pipe.lock().await.dialog_income.clone();

        tokio::spawn(async move {
            loop {
                let connected_to_realm = client_flags.lock().await.contains(
                    ClientFlags::IS_CONNECTED_TO_REALM
                );

                if let Some(packets) = input_queue.lock().await.pop_front() {
                    for packet in packets {
                        let processors = match connected_to_realm {
                            true => Self::get_realm_processors(),
                            false => Self::get_login_processors(),
                        };

                        let mut handler_input = HandlerInput {
                            session: Arc::clone(&session),
                            // packet: size + opcode + body, need to parse separately
                            data: Some(&packet),
                            data_storage: Arc::clone(&data_storage),
                            message_income: message_income.clone(),
                            dialog_income: dialog_income.clone(),
                        };

                        let handler_list = processors
                            .iter()
                            .map(|processor| processor(&mut handler_input))
                            .flatten()
                            .collect::<Vec<HandlerFunction>>();

                        for mut handler in handler_list {
                            match handler(&mut handler_input) {
                                Ok(output) => {
                                    match output {
                                        HandlerOutput::Data((opcode, header, body)) => {
                                            let packet = match opcode {
                                                Opcode::CMSG_WARDEN_DATA => {
                                                    let warden_crypt =
                                                        &mut *warden_crypt.lock().await;
                                                    [header, warden_crypt
                                                        .as_mut().unwrap().encrypt(&body)
                                                    ].concat()
                                                },
                                                _ => [header, body].concat(),
                                            };

                                            output_queue.lock().await.push_back(packet);
                                        },
                                        HandlerOutput::ConnectionRequest(host, port) => {
                                            match Self::connect_inner(&host, port).await {
                                                Ok(stream) => {
                                                    let message = format!(
                                                        "Connected to {}:{}", host, port
                                                    );

                                                    Self::set_stream_halves(
                                                        stream, &reader, &writer
                                                    ).await;

                                                    message_income.send_success_message(message);

                                                },
                                                Err(err) => {
                                                    message_income.send_error_message(
                                                        err.to_string()
                                                    );
                                                }
                                            }
                                        },
                                        HandlerOutput::UpdateState(state) => {
                                            match state {
                                                State::SetEncryption(session_key) => {
                                                    *warden_crypt.lock().await = Some(
                                                        WardenCrypt::new(&session_key)
                                                    );

                                                    if let Some(reader) = &mut *reader.lock().await
                                                    {
                                                        reader.init(
                                                            &session_key,
                                                            Arc::clone(&warden_crypt)
                                                        );
                                                    }

                                                    if let Some(writer) = &mut *writer.lock().await
                                                    {
                                                        writer.init(&session_key);
                                                    }
                                                },
                                                State::SetConnectedToRealm(is_authorized) => {
                                                    client_flags.lock().await.set(
                                                        ClientFlags::IS_CONNECTED_TO_REALM,
                                                        is_authorized
                                                    );
                                                },
                                            }
                                        },
                                        HandlerOutput::Freeze => {
                                            client_flags.lock().await.set(
                                                ClientFlags::IN_FROZEN_MODE,
                                                true
                                            );

                                            loop {
                                                let frozen_mode = client_flags
                                                    .lock().await
                                                    .contains(ClientFlags::IN_FROZEN_MODE);

                                                if !frozen_mode {
                                                    break;
                                                }

                                                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
                                            }
                                        },
                                        HandlerOutput::Void => {},
                                    };
                                },
                                Err(err) => {
                                    message_income.send_error_message(err.to_string());
                                },
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
        let output_queue = Arc::clone(&self._output_queue);
        let writer = Arc::clone(&self._writer);
        let mut message_income = self._income_message_pipe.lock().await.message_income.clone();

        tokio::spawn(async move {
            loop {
                if let Some(packet) = output_queue.lock().await.pop_front() {
                    if !packet.is_empty() {
                        match &mut *writer.lock().await {
                            Some(writer) => {
                                match writer.write(&packet).await {
                                    Ok(_) => {},
                                    Err(err) => {
                                        message_income.send_error_message(err.to_string());
                                    }
                                };
                            },
                            None => {
                                message_income.send_error_message(
                                    String::from("Not connected to TCP")
                                );
                            },
                        };
                    }
                }

                sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
            }
        })
    }

    async fn handle_read(&mut self) -> JoinHandle<()> {
        let input_queue = Arc::clone(&self._input_queue);
        let reader = Arc::clone(&self._reader);
        let mut message_income = self._income_message_pipe.lock().await.message_income.clone();

        tokio::spawn(async move {
            loop {
                match &mut *reader.lock().await {
                    Some(reader) => {
                        if let Some(packets) = reader.read().await.ok() {
                            input_queue.lock().await.push_back(packets);
                        }
                    },
                    None => {
                        message_income.send_error_message(String::from("Not connected to TCP"));
                    },
                };

                sleep(Duration::from_millis(READ_TIMEOUT)).await;
            }
        })
    }

    fn get_login_processors() -> Vec<ProcessorFunction> {
        return vec![
            Box::new(AuthProcessor::process_input),
        ];
    }

    fn get_realm_processors() -> Vec<ProcessorFunction> {
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
    use crate::ipc::session::types::{ActionFlags, StateFlags};

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

        let reader = &mut *client._reader.lock().await;
        assert!(reader.is_none());

        let writer = &mut *client._writer.lock().await;
        assert!(writer.is_none());

        let warden_crypt = &mut *client._warden_crypt.lock().await;
        assert!(warden_crypt.is_none());

        let client_flags = &mut *client.client_flags.lock().await;
        assert_eq!(ClientFlags::NONE, *client_flags);

        let input_queue = &mut *client._input_queue.lock().await;
        assert!(input_queue.is_empty());

        let output_queue = &mut *client._output_queue.lock().await;
        assert!(output_queue.is_empty());

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
                    if let Some(packet) = client._input_queue.lock().await.pop_front() {
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
            client._output_queue.lock().await.push_back(PACKET.to_vec());

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