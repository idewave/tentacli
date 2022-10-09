use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::{Mutex, mpsc};
use tokio::net::TcpStream;
use tokio::task::{JoinHandle};
use futures::future::join_all;
use tokio::sync::mpsc::{Receiver, Sender};
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
use crate::ipc::pipe::dialog::DialogIncome;
use crate::ipc::pipe::message::MessageIncome;
use crate::ipc::pipe::types::Signal;
use crate::ipc::storage::DataStorage;
use crate::ipc::session::Session;
use crate::network::stream::{Reader, Writer};
use crate::types::traits::Processor;
use crate::types::{AIManagerInput, HandlerInput, HandlerOutput, ProcessorFunction, ProcessorResult};
use crate::UI;
use crate::ui::types::{UIOutputOptions, UIRenderOptions};
use crate::ui::{UIInput, UIOutput};

const WRITE_TIMEOUT: u64 = 1;

pub struct Client {
    _reader: Arc<Mutex<Option<Reader>>>,
    _writer: Arc<Mutex<Option<Writer>>>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _income_message_pipe: Arc<SyncMutex<IncomeMessagePipe>>,
    _outcome_message_pipe: Arc<SyncMutex<OutcomeMessagePipe>>,

    session: Arc<SyncMutex<Session>>,
    data_storage: Arc<SyncMutex<DataStorage>>,
    client_flags: Arc<SyncMutex<ClientFlags>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            _reader: Arc::new(Mutex::new(None)),
            _writer: Arc::new(Mutex::new(None)),
            _warden_crypt: Arc::new(SyncMutex::new(None)),
            _income_message_pipe: Arc::new(SyncMutex::new(IncomeMessagePipe::new())),
            _outcome_message_pipe: Arc::new(SyncMutex::new(OutcomeMessagePipe::new())),

            session: Arc::new(SyncMutex::new(Session::new())),
            data_storage: Arc::new(SyncMutex::new(DataStorage::new())),
            client_flags: Arc::new(SyncMutex::new(ClientFlags::NONE)),
        }
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<(), Error> {
        let mut income_pipe = self._income_message_pipe.lock().unwrap();

        return match Self::connect_inner(host, port).await {
            Ok(stream) => {
                Self::set_stream_halves(
                    stream,
                    Arc::clone(&self._reader),
                    Arc::clone(&self._writer),
                    None,
                    Arc::clone(&self._warden_crypt),
                ).await;

                match self.session.lock().unwrap().set_config(host) {
                    Ok(_) => {},
                    Err(err) => {
                        income_pipe.message_income.send_error_message(err.to_string());
                    }
                }

                income_pipe.message_income.send_success_message(
                    format!("Connected to {}:{}", host, port)
                );

                Ok(())
            },
            Err(err) => {
                income_pipe.message_income.send_error_message(
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

    pub async fn run(&mut self) {
        const BUFFER_SIZE: usize = 50;

        let (signal_sender, signal_receiver) = mpsc::channel::<Signal>(1);
        let (input_sender, input_receiver) = mpsc::channel::<Vec<u8>>(BUFFER_SIZE);
        let (output_sender, output_receiver) = mpsc::channel::<Vec<u8>>(BUFFER_SIZE);

        let message_income = self._income_message_pipe.lock().unwrap().message_income.clone();
        let dialog_income = self._income_message_pipe.lock().unwrap().dialog_income.clone();

        let username = {
            let guard = self.session.lock().unwrap();
            let config = guard.get_config().unwrap();

            config.connection_data.username.clone()
        };

        {
            let mut guard = self._income_message_pipe.lock().unwrap();
            guard.message_income.send_client_message(
                format!("LOGIN_CHALLENGE as {}", &username)
            );
        }

        output_sender.send(login_challenge(&username)).await.unwrap();

        join_all(vec![
            self.handle_ui_render(),
            self.handle_ui_input(),
            self.handle_ui_output(),
            self.handle_ai(output_sender.clone(), message_income.clone()),
            self.handle_read(input_sender.clone(), signal_receiver, message_income.clone()),
            self.handle_write(output_receiver, message_income.clone()),
            self.handle_packets(
                input_receiver,
                signal_sender.clone(),
                output_sender.clone(),
                message_income.clone(),
                dialog_income.clone(),
            ),
        ]).await;
    }

    fn handle_ui_input(&mut self) -> JoinHandle<()> {
        let event_income = self._income_message_pipe.lock().unwrap().event_income.clone();

        tokio::task::spawn_blocking(move || {
            let mut ui_input = UIInput::new(event_income);

            loop {
                ui_input.handle();
            }
        })
    }

    fn handle_ui_render(&mut self) -> JoinHandle<()> {
        let income_message_pipe = Arc::clone(&self._income_message_pipe);
        let dialog_outcome = self._outcome_message_pipe.lock().unwrap().dialog_outcome.clone();
        let flag_outcome = self._outcome_message_pipe.lock().unwrap().flag_outcome.clone();
        let client_flags = Arc::clone(&self.client_flags);

        tokio::task::spawn_blocking(move || {
            let mut ui = UI::new(
                CrosstermBackend::new(std::io::stdout()),
                UIOutputOptions {
                    dialog_outcome,
                    flag_outcome,
                },
            );

            loop {
                let client_flags = {
                    let guard = client_flags.lock().unwrap();
                    guard.clone()
                };

                ui.render(UIRenderOptions {
                    message: income_message_pipe.lock().unwrap().recv(),
                    client_flags: &client_flags,
                });
            }
        })
    }

    fn handle_ui_output(&mut self) -> JoinHandle<()> {
        let outcome_message_pipe = Arc::clone(&self._outcome_message_pipe);
        let session = Arc::clone(&self.session);
        let client_flags = Arc::clone(&self.client_flags);

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(message) = outcome_message_pipe.lock().unwrap().recv() {
                    let client_flags = &mut *client_flags.lock().unwrap();

                    let mut ui_output = UIOutput::new(Arc::clone(&session), client_flags);
                    ui_output.handle(message);
                }
            }
        })
    }

    fn handle_ai(
        &mut self,
        mut output_sender: Sender<Vec<u8>>,
        mut message_income: MessageIncome,
    ) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let data_storage = Arc::clone(&self.data_storage);

        let mut movement_ai = MovementAI::new();

        tokio::spawn(async move {
            loop {
                let input = AIManagerInput {
                    session: Arc::clone(&session),
                    data_storage: Arc::clone(&data_storage),
                    output_sender: output_sender.clone(),
                    message_income: message_income.clone(),
                };

                movement_ai.manage(input).await;
            }
        })
    }

    fn handle_packets(
        &mut self,
        mut input_receiver: Receiver<Vec<u8>>,
        mut signal_sender: Sender<Signal>,
        mut output_sender: Sender<Vec<u8>>,
        mut message_income: MessageIncome,
        mut dialog_income: DialogIncome,
    ) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let reader = Arc::clone(&self._reader);
        let writer = Arc::clone(&self._writer);
        let warden_crypt = Arc::clone(&self._warden_crypt);
        let client_flags = Arc::clone(&self.client_flags);
        let data_storage = Arc::clone(&self.data_storage);

        tokio::spawn(async move {
            loop {
                let packet = input_receiver.recv().await;
                if packet.is_some() {
                    let packet = packet.unwrap();

                    let processors = {
                        let guard = client_flags.lock().unwrap();
                        let connected_to_realm = guard.contains(ClientFlags::IS_CONNECTED_TO_REALM);

                        match connected_to_realm {
                            true => Self::get_realm_processors(),
                            false => Self::get_login_processors(),
                        }
                    };

                    let session_key = {
                        let guard = session.lock().unwrap();
                        guard.session_key.clone()
                    };

                    let mut input = HandlerInput {
                        session: Arc::clone(&session),
                        // packet: size + opcode + body, need to parse separately
                        data: Some(&packet),
                        data_storage: Arc::clone(&data_storage),
                        message_income: message_income.clone(),
                        dialog_income: dialog_income.clone(),
                    };

                    let handler_list = processors
                        .iter()
                        .map(|processor| processor(&mut input))
                        .flatten()
                        .collect::<ProcessorResult>();

                    for mut handler in handler_list {
                        let response = handler.handle(&mut input).await;
                        match response {
                            Ok(output) => {
                                match output {
                                    HandlerOutput::Data((opcode, header, body)) => {
                                        message_income.send_client_message(
                                            Opcode::get_opcode_name(opcode)
                                        );
                                        let body = match opcode {
                                            Opcode::CMSG_WARDEN_DATA => {
                                                warden_crypt.lock().unwrap().as_mut().unwrap().encrypt(&body)
                                            },
                                            _ => body,
                                        };

                                        let packet = [header, body].concat();
                                        output_sender.send(packet).await.unwrap();
                                    },
                                    HandlerOutput::ConnectionRequest(host, port) => {
                                        match Self::connect_inner(&host, port).await {
                                            Ok(stream) => {
                                                signal_sender.send(Signal::Reconnect).await;

                                                Self::set_stream_halves(
                                                    stream,
                                                    Arc::clone(&reader),
                                                    Arc::clone(&writer),
                                                    session_key.clone(),
                                                    Arc::clone(&warden_crypt),
                                                ).await;

                                                message_income.send_success_message(
                                                    format!("Connected to {}:{}", host, port)
                                                );

                                                client_flags.lock().unwrap().set(
                                                    ClientFlags::IS_CONNECTED_TO_REALM,
                                                    true,
                                                );
                                            },
                                            Err(err) => {
                                                message_income.send_error_message(
                                                    err.to_string()
                                                );
                                            }
                                        }
                                    },
                                    HandlerOutput::Freeze => {
                                        {
                                            client_flags.lock().unwrap().set(
                                                ClientFlags::IN_FROZEN_MODE,
                                                true
                                            );
                                        }

                                        loop {
                                            let frozen_mode = client_flags
                                                .lock()
                                                .unwrap()
                                                .contains(ClientFlags::IN_FROZEN_MODE);

                                            if !frozen_mode {
                                                break;
                                            }
                                        }
                                    },
                                    HandlerOutput::Void => {},
                                };
                            },
                            Err(err) => {
                                message_income.send_error_message(err.to_string());
                            },
                        };
                    }
                }
            }
        })
    }

    fn handle_write(
        &mut self,
        mut output_receiver: Receiver<Vec<u8>>,
        mut message_income: MessageIncome,
    ) -> JoinHandle<()> {
        let writer = Arc::clone(&self._writer);

        tokio::spawn(async move {
            loop {
                let packet = output_receiver.recv().await;
                if packet.is_some() {
                    let packet = packet.unwrap();
                    if !packet.is_empty() {
                        let result = Self::write_packet(&writer, packet).await;

                        match result {
                            Ok(bytes_sent) => {
                                message_income.send_debug_message(
                                    format!("{} bytes sent", bytes_sent)
                                );
                            },
                            Err(err) => {
                                message_income.send_client_message(err.to_string());
                            }
                        }
                    }
                }
            }
        })
    }

    fn handle_read(
        &mut self,
        mut input_sender: Sender<Vec<u8>>,
        mut signal_receiver: Receiver<Signal>,
        mut message_income: MessageIncome,
    ) -> JoinHandle<()> {
        let reader = Arc::clone(&self._reader);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    signal = signal_receiver.recv() => {},
                    result = Self::read_packet(&reader) => {
                        match result {
                            Ok(packets) => {
                                input_sender.send(packets).await;
                            },
                            Err(err) => {
                                message_income.send_error_message(err.to_string());
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

        match &mut *writer.lock().await {
            Some(writer) => {
                match writer.write(&packet).await {
                    Ok(bytes_sent) => {
                        return Ok(bytes_sent);
                    },
                    Err(err) => {
                        error = err;
                    }
                };
            },
            _ => {},
        };

        Err(error)
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
    use std::time::Duration;
    use futures::FutureExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;

    use crate::Client;
    use crate::client::types::ClientFlags;
    use crate::client::WRITE_TIMEOUT;
    use crate::ipc::pipe::dialog::DialogIncome;
    use crate::ipc::pipe::message::MessageIncome;
    use crate::ipc::pipe::types::{IncomeMessageType, Signal};
    use crate::ipc::session::types::{ActionFlags, StateFlags};
    use crate::types::PacketList;

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

        let warden_crypt = &mut *client._warden_crypt.lock().unwrap();
        assert!(warden_crypt.is_none());

        let client_flags = &mut *client.client_flags.lock().unwrap();
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

            let (signal_sender, signal_receiver) = mpsc::channel::<Signal>(1);
            let (input_sender, mut input_receiver) = mpsc::channel::<Vec<u8>>(1);
            let (output_sender, output_receiver) = mpsc::channel::<Vec<u8>>(1);
            let (input_tx, input_rx) = std::sync::mpsc::channel::<IncomeMessageType>();

            let message_income = MessageIncome::new(input_tx.clone());
            let dialog_income = DialogIncome::new(input_tx.clone());

            if let Some((mut stream, _)) = listener.accept().await.ok() {
                stream.write(&PACKET).await.unwrap();
                stream.flush().await.unwrap();

                let read_task = client.handle_read(input_sender, signal_receiver, message_income);

                let test_task = tokio::spawn(async move {
                    loop {
                        // let packet = client._input_queue.lock().unwrap().pop_front();
                        let packet = input_receiver.recv().await;
                        if packet.is_some() {
                            assert_eq!(PACKET.to_vec(), packet.unwrap());
                            break;
                        }
                        tokio::time::sleep(Duration::from_millis(WRITE_TIMEOUT)).await;
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
            // client._output_queue.lock().unwrap().push_back(PACKET.to_vec());

            let (signal_sender, signal_receiver) = mpsc::channel::<Signal>(1);
            let (input_sender, mut input_receiver) = mpsc::channel::<PacketList>(1);
            let (mut output_sender, output_receiver) = mpsc::channel::<Vec<u8>>(1);
            let (input_tx, input_rx) = std::sync::mpsc::channel::<IncomeMessageType>();

            let message_income = MessageIncome::new(input_tx.clone());
            let dialog_income = DialogIncome::new(input_tx.clone());

            output_sender.send(PACKET.to_vec()).await.unwrap();

            if let Some((stream, _)) = listener.accept().await.ok() {
                let buffer_size = PACKET.to_vec().len();
                let mut buffer = Vec::with_capacity(buffer_size);

                client.handle_write(output_receiver, message_income);
                stream.take(buffer_size as u64).read_to_end(&mut buffer).await.unwrap();

                assert_eq!(PACKET.to_vec(), buffer);
            }
        }
    }
}