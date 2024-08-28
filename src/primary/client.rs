#![allow(clippy::new_without_default)]
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
use tentacli_crypto::{Decryptor, Encryptor, WardenCrypt};
use tentacli_traits::{Feature};
use tentacli_traits::types::config::{EnvConfig, EnvConfigParams};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::{DataStorage, Session};
use tentacli_traits::types::{
    HandlerInput, HandlerOutput, IncomingPacket,
    OutgoingPacket, ProcessorResult, Signal
};
use tentacli_utils::encode_hex;

use crate::primary::network::{Reader, Writer};

#[derive(Default)]
pub struct CreateOptions {
    pub data_storage: Option<Arc<SyncMutex<DataStorage>>>
}

pub struct RunOptions<'a> {
    pub external_features: Vec<Box<dyn Feature>>,
    pub config_path: &'a str,
    pub account: &'a str,
    pub dotenv_path: &'a str,
}

pub struct Client {
    _reader: Arc<Mutex<Option<Reader>>>,
    _writer: Arc<Mutex<Option<Writer>>>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,

    session: Arc<Mutex<Session>>,
    data_storage: Arc<SyncMutex<DataStorage>>,
}

impl Client {
    pub fn new(options: CreateOptions) -> Self {
        Self {
            _reader: Arc::new(Mutex::new(None)),
            _writer: Arc::new(Mutex::new(None)),
            _warden_crypt: Arc::new(SyncMutex::new(None)),

            session: Arc::new(Mutex::new(Session::new())),
            data_storage: options.data_storage
                .unwrap_or_else(|| Arc::new(SyncMutex::new(DataStorage::default()))),
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
            *reader.lock().await = Some(
                Reader::new(rx, Arc::new(SyncMutex::new(None)), false, None)
            );
            *writer.lock().await = Some(
                Writer::new(tx, Arc::new(SyncMutex::new(None)), false, None)
            );
        } else {
            let session_key = session_key.unwrap();
            *warden_crypt.lock().unwrap() = Some(WardenCrypt::new(&session_key));

            *reader.lock().await = Some(
                Reader::new(
                    rx,
                    Arc::clone(&warden_crypt),
                    true,
                    Some(Decryptor::new(&session_key))
                )
            );

            *writer.lock().await = Some(
                Writer::new(
                    tx,
                    Arc::clone(&warden_crypt),
                    true,
                    Some(Encryptor::new(&session_key))
                )
            );
        }
    }

    pub async fn run<'a>(&mut self, options: RunOptions<'a>) -> AnyResult<()> {
        let EnvConfig { host, port } = EnvConfig::new(
            EnvConfigParams { dotenv_path: options.dotenv_path }
        )?;

        const BUFFER_SIZE: usize = 50;

        let notify = Arc::new(Notify::new());

        let (signal_sender, signal_receiver) = mpsc::channel::<Signal>(1);
        let (output_sender, output_receiver) = mpsc::channel::<OutgoingPacket>(BUFFER_SIZE);
        let (query_sender, query_receiver) = broadcast::<HandlerOutput>(BUFFER_SIZE);

        match Self::connect_inner(&host, port).await {
            Ok(stream) => {
                Self::set_stream_halves(
                    stream,
                    Arc::clone(&self._reader),
                    Arc::clone(&self._writer),
                    None,
                    Arc::clone(&self._warden_crypt),
                ).await;

                match self.session.lock().await.set_config(&host, options.account, options.config_path) {
                    Ok(_) => {},
                    Err(err) => {
                        query_sender.broadcast(
                            HandlerOutput::ErrorMessage(err.to_string(), None)
                        ).await.unwrap();
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
                query_sender.broadcast(
                    HandlerOutput::ErrorMessage(format!("Cannot connect: {}", err), None)
                ).await.unwrap();

                Err(err)
            },
        }?;

        #[allow(unused_mut)]
        let mut features: Vec<Box<dyn Feature>> = options.external_features;
        cfg_if! {
            if #[cfg(feature = "ui")] {
                use crate::features::ui::UI;

                features.push(Box::new(UI::default()));
            } else if #[cfg(feature = "console")] {
                use crate::features::console::Console;

                features.push(Box::new(Console::default()));
            }
        }

        cfg_if! {
            if #[cfg(feature = "wotlk_login")] {
                use crate::features::wotlk_login::WotlkLogin;
                features.push(Box::new(WotlkLogin::default()));
            }
        }

        cfg_if! {
            if #[cfg(feature = "wotlk_realm")] {
                use crate::features::wotlk_realm::WotlkRealm;
                features.push(Box::new(WotlkRealm));
            }
        }

        for feature in &mut features {
            feature.set_broadcast_channel(query_sender.clone(), query_receiver.clone());
        }

        let mut all_tasks = vec![];

        for feature in features.iter_mut() {
            match feature.get_tasks() {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => eprintln!("Error on get_tasks: {:?}", e),
            }
        }

        all_tasks.extend(vec![
            self.handle_read(signal_receiver, query_sender.clone(), notify.clone(), features),
            self.handle_output(
                signal_sender.clone(), output_sender.clone(), query_sender.clone(),
                query_receiver, notify.clone(),
            ),
            self.handle_write(output_receiver, query_sender),
        ]);

        join_all(all_tasks).await;

        Ok(())
    }

    fn handle_read(
        &mut self,
        mut signal_receiver: Receiver<Signal>,
        query_sender: BroadcastSender<HandlerOutput>,
        notify: Arc<Notify>,
        features: Vec<Box<dyn Feature>>,
    ) -> JoinHandle<()> {
        let reader = Arc::clone(&self._reader);
        let session = Arc::clone(&self.session);
        let data_storage = Arc::clone(&self.data_storage);

        tokio::spawn(async move {
            let mut realm_processors = vec![];
            let mut processors = vec![];
            let mut one_time_handler_maps = vec![];
            let mut initial_processors = vec![];

            for feature in features.into_iter() {
                realm_processors.extend(feature.get_realm_processors());
                processors.extend(feature.get_login_processors());
                one_time_handler_maps.extend(feature.get_one_time_handler_maps());
                initial_processors.extend(feature.get_initial_processors());
            }

            let mut realm_processors = Some(realm_processors);

            let handler_list = initial_processors
                .iter()
                .flat_map(|processor| processor(Opcode::LOGIN_CHALLENGE as u16))
                .collect::<ProcessorResult>();

            Self::call_handlers(
                handler_list, &query_sender, &notify, HandlerInput {
                    session: Arc::clone(&session),
                    data: vec![],
                    data_storage: Arc::clone(&data_storage),
                    opcode: Opcode::LOGIN_CHALLENGE as u16,
                }
            ).await;

            loop {
                tokio::select! {
                    _ = signal_receiver.recv() => {
                        // realm_processors will be None on next iteration
                        // so this approach ensures that realm_processors will be taken only once,
                        // but it seems I still can use it in current iteration
                        processors = realm_processors.take().unwrap();
                    },
                    result = Self::read_packet(&reader) => {
                        match result {
                            Ok(packet) => {
                                let IncomingPacket { opcode, body: data, .. } = packet;

                                let input = HandlerInput {
                                    session: Arc::clone(&session),
                                    data,
                                    data_storage: Arc::clone(&data_storage),
                                    opcode,
                                };

                                let mut handler_list = processors
                                    .iter()
                                    .flat_map(|processor| processor(opcode))
                                    .collect::<ProcessorResult>();

                                for map in one_time_handler_maps.iter_mut() {
                                    if let Some(handlers) = map.remove(&opcode) {
                                        handler_list.extend(handlers)
                                    }
                                }

                                if handler_list.is_empty() {
                                    let opcode_name = Opcode::get_opcode_name(
                                        input.opcode as u32
                                    ).unwrap_or(format!("Unknown opcode: {}", input.opcode));

                                    query_sender.broadcast(HandlerOutput::ResponseMessage(
                                        opcode_name,
                                        Some(encode_hex(&input.data)),
                                    )).await.unwrap();
                                }

                                Self::call_handlers(
                                    handler_list, &query_sender, &notify, input
                                ).await;
                            },
                            Err(err) => {
                                query_sender.broadcast(
                                    HandlerOutput::ErrorMessage(err.to_string(), None)
                                ).await.unwrap();
                                sleep(Duration::from_secs(1)).await;
                            }
                        }
                    },
                }
            }
        })
    }

    async fn call_handlers(
        handler_list: ProcessorResult,
        query_sender: &BroadcastSender<HandlerOutput>,
        notify: &Arc<Notify>,
        mut input: HandlerInput
    ) {
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
                    query_sender.broadcast(
                        HandlerOutput::ErrorMessage(err.to_string(), None)
                    ).await.unwrap();
                },
            };
        }
    }

    fn handle_output(
        &mut self,
        signal_sender: Sender<Signal>,
        output_sender: Sender<OutgoingPacket>,
        query_sender: BroadcastSender<HandlerOutput>,
        mut query_receiver: BroadcastReceiver<HandlerOutput>,
        notify: Arc<Notify>,
    ) -> JoinHandle<()> {
        let session = Arc::clone(&self.session);
        let reader = Arc::clone(&self._reader);
        let writer = Arc::clone(&self._writer);
        let warden_crypt = Arc::clone(&self._warden_crypt);

        tokio::spawn(async move {
            loop {
                let result = query_receiver.recv().await;
                match result {
                    Ok(output) => {
                        match output {
                            HandlerOutput::Data((opcode, data, json_details)) => {
                                output_sender.send(OutgoingPacket {
                                    opcode,
                                    data,
                                    json_details,
                                }).await.unwrap();
                            },
                            HandlerOutput::ConnectionRequest(host, port) => {
                                match Self::connect_inner(&host, port).await {
                                    Ok(stream) => {
                                        signal_sender.send(Signal::Reconnect).await.unwrap();

                                        let session_key = {
                                            let guard = session.lock().await;
                                            let srp = guard.srp.as_ref().unwrap();
                                            srp.session_key.to_vec()
                                        };

                                        Self::set_stream_halves(
                                            stream,
                                            Arc::clone(&reader),
                                            Arc::clone(&writer),
                                            Some(session_key.clone()),
                                            Arc::clone(&warden_crypt),
                                        ).await;

                                        query_sender.broadcast(
                                            HandlerOutput::SuccessMessage(
                                                format!("Connected to {}:{}", host, port),
                                                None
                                            )
                                        ).await.unwrap();
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
                                session.lock().await.selected_realm = Some(realm);
                                notify.notify_one();
                            },
                            HandlerOutput::SelectCharacter(character) => {
                                session.lock().await.me = Some(character);
                                notify.notify_one();
                            },
                            _ => {},
                        };
                    },
                    Err(err) => {
                        query_sender.broadcast(
                            HandlerOutput::ErrorMessage(err.to_string(), None)
                        ).await.unwrap();
                    },
                };
            }
        })
    }

    fn handle_write(
        &mut self,
        mut output_receiver: Receiver<OutgoingPacket>,
        query_sender: BroadcastSender<HandlerOutput>,
    ) -> JoinHandle<()> {
        let writer = Arc::clone(&self._writer);

        tokio::spawn(async move {
            loop {
                if let Some(packet) = output_receiver.recv().await {
                    if !packet.data.is_empty() {
                        let result = Self::write_packet(&writer, &packet).await;

                        match result {
                            Ok(bytes_sent) => {
                                let message = format!(
                                    "{}: {} bytes sent",
                                    Opcode::get_opcode_name(packet.opcode)
                                        .unwrap_or(packet.opcode.to_string()),
                                    bytes_sent,
                                );

                                query_sender.broadcast(
                                    HandlerOutput::RequestMessage(
                                        message, Some(packet.json_details)
                                    )
                                ).await.unwrap();
                            },
                            Err(err) => {
                                query_sender.broadcast(
                                    HandlerOutput::ErrorMessage(err.to_string(), None)
                                ).await.unwrap();
                            }
                        }
                    }
                }
            }
        })
    }

    async fn read_packet(reader: &Arc<Mutex<Option<Reader>>>) -> AnyResult<IncomingPacket> {
        let error = Error::new(ErrorKind::NotFound, "Not connected to TCP");

        if let Some(reader) = &mut *reader.lock().await {
            return match reader.read().await {
                Ok(packet) => Ok(packet),
                Err(err) => Err(err),
            };
        }

        Err(anyhow::Error::new(error))
    }

    async fn write_packet(
        writer: &Arc<Mutex<Option<Writer>>>,
        packet: &OutgoingPacket
    ) -> AnyResult<usize> {
        let error = Error::new(ErrorKind::NotFound, "Not connected to TCP");

        if let Some(writer) = &mut *writer.lock().await {
            return match writer.write(packet).await {
                Ok(bytes_sent) => Ok(bytes_sent),
                Err(err) => Err(err)
            };
        }

        Err(anyhow::Error::new(error))
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
    use tokio::io::{AsyncReadExt};
    use tokio::net::TcpListener;
    use tokio::sync::{mpsc};
    use tentacli_traits::types::{HandlerOutput, OutgoingPacket};
    use tentacli_traits::types::shared::{ActionFlags, StateFlags};

    use crate::primary::client::{Client, CreateOptions};

    const HOST: &str = "127.0.0.1";
    // https://users.rust-lang.org/t/async-tests-sometimes-fails/78451
    // port should be zero to avoid race condition (in case of running in parallel)
    // so OS will create connection with random port
    const PORT: u16 = 0;
    const PACKET: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    #[tokio::test]
    async fn test_client_create() {
        let client = Client::new(CreateOptions::default());

        let reader = &mut *client._reader.lock().await;
        assert!(reader.is_none());

        let writer = &mut *client._writer.lock().await;
        assert!(writer.is_none());

        let warden_crypt = &mut *client._warden_crypt.lock().unwrap();
        assert!(warden_crypt.is_none());

        let data_storage = &mut *client.data_storage.lock().unwrap();
        assert!(data_storage.players_map.is_empty());

        let session = &mut *client.session.lock().await;
        assert!(session.srp.is_none());
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
        let mut client = Client::new(CreateOptions::default());
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
    async fn test_client_write_outcoming_data() {
        let mut client = Client::new(CreateOptions::default());
        if let Some(listener) = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.ok() {
            let local_addr = listener.local_addr().unwrap();
            client.connect(HOST, local_addr.port()).await.ok();

            let (output_sender, output_receiver) = mpsc::channel::<OutgoingPacket>(1);
            let (query_sender, _) = broadcast::<HandlerOutput>(1);

            output_sender.send(
                OutgoingPacket { opcode: 0, data: PACKET.to_vec(), json_details: String::new() }
            ).await.unwrap();

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