#![allow(clippy::new_without_default)]

use std::io::Error;
use std::sync::{Arc, Mutex as SyncMutex};
use std::time::Duration;

use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use futures::future::join_all;
use tentacli_crypto::{Decryptor, Encryptor, WardenCrypt};
use tentacli_traits::Feature;
use tentacli_traits::types::{HandlerInput, HandlerOutput, IncomingPacket, OutgoingPacket, ProcessorResult, Signal, Task};
use tentacli_traits::types::config::{EnvConfig, EnvConfigParams};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::{DataStorage, Session};
use tentacli_utils::encode_hex;
#[cfg(feature = "relay")]
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::sleep;

use crate::primary::network::{Reader, Writer};

const BUFFER_SIZE: usize = 100;

#[derive(Default)]
pub struct CreateOptions {
    pub data_storage: Option<Arc<Mutex<DataStorage>>>,
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
    data_storage: Arc<Mutex<DataStorage>>,
}

impl Client {
    pub fn new(options: CreateOptions) -> Self {
        Self {
            _reader: Arc::new(Mutex::new(None)),
            _writer: Arc::new(Mutex::new(None)),
            _warden_crypt: Arc::new(SyncMutex::new(None)),

            session: Arc::new(Mutex::new(Session::default())),
            data_storage: options.data_storage
                .unwrap_or_else(|| Arc::new(Mutex::new(DataStorage::default()))),
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

        match session_key {
            Some(session_key) => {
                *warden_crypt.lock().unwrap() = Some(WardenCrypt::new(&session_key));

                *reader.lock().await = Some(
                    Reader::new(
                        rx,
                        Arc::clone(&warden_crypt),
                        true,
                        Some(Decryptor::new(&session_key)),
                    )
                );

                *writer.lock().await = Some(
                    Writer::new(
                        tx,
                        Arc::clone(&warden_crypt),
                        true,
                        Some(Encryptor::new(&session_key)),
                    )
                );
            }
            None => {
                *reader.lock().await = Some(
                    Reader::new(rx, Arc::new(SyncMutex::new(None)), false, None)
                );
                *writer.lock().await = Some(
                    Writer::new(tx, Arc::new(SyncMutex::new(None)), false, None)
                );
            }
        }
    }

    pub async fn run(&mut self, options: RunOptions<'_>) -> anyhow::Result<()> {
        let RunOptions { account, config_path, dotenv_path, external_features } = options;
        let EnvConfig { host, port } = EnvConfig::new(
            EnvConfigParams { dotenv_path }
        )?;

        let notify = Arc::new(Notify::new());

        let (signal_sender, signal_receiver) = broadcast::<Signal>(1);
        let (output_sender, output_receiver) = mpsc::channel::<OutgoingPacket>(BUFFER_SIZE);
        #[cfg(feature = "relay")]
        let (incoming_sender, incoming_receiver) = mpsc::channel::<IncomingPacket>(BUFFER_SIZE);
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

                self.session.lock().await.set_config(&host, account, config_path)?;

                query_sender.broadcast(
                    HandlerOutput::SuccessMessage(
                        format!("Connected to {}:{}", host, port),
                        None,
                    )
                ).await?;

                Ok(())
            }
            Err(err) => {
                query_sender.broadcast(
                    HandlerOutput::ErrorMessage(format!("Cannot connect: {}", err), None)
                ).await?;

                Err(err)
            }
        }?;

        #[allow(unused_mut)]
        let mut features: Vec<Box<dyn Feature>> = external_features;
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
            all_tasks.extend(feature.get_tasks()?);
        }

        all_tasks.extend(vec![
            self.handle_read(
                signal_receiver.clone(),
                query_sender.clone(),
                #[cfg(feature = "relay")]
                incoming_sender.clone(),
                notify.clone(),
                features,
            ),
            self.handle_output(
                signal_sender.clone(),
                output_sender.clone(),
                query_sender.clone(),
                query_receiver,
                notify.clone(),
            ),
            self.handle_write(
                output_receiver,
                query_sender,
            ),
        ]);

        cfg_if! {
            if #[cfg(feature = "relay")] {
                all_tasks.push(
                    self.handle_external_write(
                        incoming_receiver,
                        signal_receiver,
                    )
                );
            }
        }

        join_all(all_tasks).await;

        Ok(())
    }

    fn handle_read(
        &mut self,
        mut signal_receiver: BroadcastReceiver<Signal>,
        query_sender: BroadcastSender<HandlerOutput>,
        #[cfg(feature = "relay")]
        incoming_sender: Sender<IncomingPacket>,
        notify: Arc<Notify>,
        features: Vec<Box<dyn Feature>>,
    ) -> Task {
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
                },
            ).await?;

            loop {
                tokio::select! {
                    _ = signal_receiver.recv() => {
                        // realm_processors will be None on next iteration
                        // so this approach ensures that realm_processors will be taken only once,
                        // but it seems I still can use it in current iteration
                        processors = realm_processors.take().unwrap();
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    },
                    result = Self::read_packet(reader.clone()) => {
                        match result {
                            Ok(packet) => {
                                let IncomingPacket { opcode, body, .. } = packet.clone();

                                let input = HandlerInput {
                                    session: Arc::clone(&session),
                                    data: body,
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

                                // if true then there no handlers defined for the current opcode
                                // in this case we just show the raw packet (hex)
                                if handler_list.is_empty() {
                                    let opcode_name = Opcode::get_opcode_name(
                                        input.opcode as u32
                                    ).unwrap_or(format!("Unknown opcode: {}", input.opcode));

                                    query_sender.broadcast(HandlerOutput::ResponseMessage(
                                        opcode_name,
                                        Some(encode_hex(&input.data)),
                                    )).await?;
                                }

                                Self::call_handlers(
                                    handler_list, &query_sender, &notify, input
                                ).await?;

                                #[cfg(feature = "relay")]
                                incoming_sender.send(packet).await?;
                            },
                            Err(err) => {
                                query_sender.broadcast(
                                    HandlerOutput::ErrorMessage(err.to_string(), None)
                                ).await?;
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
        mut input: HandlerInput,
    ) -> anyhow::Result<()> {
        for mut handler in handler_list {
            let response = handler.handle(&mut input).await;
            match response {
                Ok(outputs) => {
                    for output in outputs {
                        match output {
                            HandlerOutput::Freeze => {
                                notify.notified().await;
                            }
                            _ => {
                                query_sender.broadcast(output).await?;
                            }
                        }
                    }
                }
                Err(err) => {
                    query_sender.broadcast(
                        HandlerOutput::ErrorMessage(err.to_string(), None)
                    ).await?;
                }
            };
        }

        Ok(())
    }

    fn handle_output(
        &mut self,
        signal_sender: BroadcastSender<Signal>,
        output_sender: Sender<OutgoingPacket>,
        query_sender: BroadcastSender<HandlerOutput>,
        mut query_receiver: BroadcastReceiver<HandlerOutput>,
        notify: Arc<Notify>,
    ) -> Task {
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
                                }).await?;
                            }
                            HandlerOutput::ConnectionRequest(host, port) => {
                                match Self::connect_inner(&host, port).await {
                                    Ok(stream) => {
                                        signal_sender.broadcast(Signal::Reconnect).await?;

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
                                                None,
                                            )
                                        ).await?;
                                    }
                                    Err(err) => {
                                        query_sender.broadcast(
                                            HandlerOutput::ErrorMessage(err.to_string(), None)
                                        ).await?;
                                    }
                                }
                            }
                            HandlerOutput::Drop => {
                                break;
                            }
                            HandlerOutput::SelectRealm(realm) => {
                                session.lock().await.selected_realm = Some(realm);
                                notify.notify_one();
                            }
                            HandlerOutput::SelectCharacter(my_guid) => {
                                // if the character is being selected manually
                                session.lock().await.my_guid = Some(my_guid);
                                notify.notify_one();
                            }
                            _ => {}
                        };
                    }
                    Err(err) => {
                        query_sender.broadcast(
                            HandlerOutput::ErrorMessage(err.to_string(), None)
                        ).await?;
                    }
                };
            }

            Ok(())
        })
    }

    fn handle_write(
        &mut self,
        mut output_receiver: Receiver<OutgoingPacket>,
        query_sender: BroadcastSender<HandlerOutput>,
    ) -> Task {
        let writer = Arc::clone(&self._writer);

        tokio::spawn(async move {
            loop {
                if let Some(mut packet) = output_receiver.recv().await {
                    if !packet.data.is_empty() {
                        let result = Self::write_packet(writer.clone(), &mut packet).await;

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
                                        message, Some(packet.json_details),
                                    )
                                ).await?;
                            }
                            Err(err) => {
                                query_sender.broadcast(
                                    HandlerOutput::ErrorMessage(err.to_string(), None)
                                ).await?;
                            }
                        }
                    }
                }
            }
        })
    }

    #[cfg(feature = "relay")]
    fn handle_external_write(
        &mut self,
        mut incoming_receiver: Receiver<IncomingPacket>,
        mut signal_receiver: BroadcastReceiver<Signal>,
    ) -> Task {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = signal_receiver.recv() => {
                        break;
                    },
                    _ = incoming_receiver.recv() => {},
                }
            }

            let mut stream = loop {
                match TcpStream::connect("127.0.0.1:3788").await {
                    Ok(stream) => {
                        break stream;
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
            };

            while let Some(incoming_packet) = incoming_receiver.recv().await {
                let IncomingPacket { header: mut packet, body, .. } = incoming_packet;
                packet.extend_from_slice(&body);
                stream.write(&packet).await?;
            }

            Ok(())
        })
    }

    async fn read_packet(reader: Arc<Mutex<Option<Reader>>>) -> anyhow::Result<IncomingPacket> {
        reader.lock().await.as_mut().unwrap().read().await
    }

    async fn write_packet(
        writer: Arc<Mutex<Option<Writer>>>,
        packet: &mut OutgoingPacket,
    ) -> anyhow::Result<usize> {
        writer.lock().await.as_mut().unwrap().write(packet).await
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
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use async_broadcast::broadcast;
    use tentacli_traits::types::{HandlerOutput, OutgoingPacket};
    use tentacli_traits::types::shared::{ActionFlags, StateFlags};
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpListener;
    use tokio::sync::mpsc;

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

        let data_storage = client.data_storage.lock().await;
        assert!(data_storage.players_map.is_empty());

        let session = &mut *client.session.lock().await;
        assert!(session.srp.is_none());
        assert!(session.warden_module_info.is_none());
        assert!(session.config.is_none());
        assert!(session.follow_target.is_none());
        assert!(session.selected_realm.is_none());
        assert!(session.my_guid.is_none());
        assert_eq!(session.action_flags, ActionFlags::NONE);
        assert_eq!(session.state_flags, StateFlags::NONE);
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