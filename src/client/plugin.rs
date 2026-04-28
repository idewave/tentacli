use async_trait::async_trait;
use futures::future::select_all;
use futures::{FutureExt, TryFutureExt};
use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::RwLock;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::client::PluginLoader;
use crate::client::packet::{BytesRead, OutputBuilder, Packet, Processor, Serializer};
use crate::client::transport::{
    Protocol, TcpRead, TcpWrite, TransportRead, TransportWrite, UdpRead, UdpWrite,
};
use crate::client::types::{
    CtxMap, Echo, HandlerOutput, Message, MsgType, OrderedOutput, Request, ServerLabel, Task,
};

#[async_trait]
pub trait NetworkPlugin: Send + Sync + Any
where
    Self: 'static,
{
    #[allow(clippy::too_many_arguments)]
    fn connect(
        self: Arc<Self>,
        mut echo_rx: Receiver<Echo>,
        echo_tx: Sender<Echo>,
        packet_tx: Sender<Vec<Packet>>,
        packet_rx: Receiver<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        shutdown: CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> Task {
        let plugin = self.clone();

        tokio::spawn(async move {
            // The remote address can be static or dynamic.
            // A static address is defined directly.
            // A dynamic address is provided by Request::Connect and shared with Echo::Connect.
            let remote_addr: String = match plugin.remote_addr()? {
                Some(addr) => addr.to_string(),
                None => loop {
                    let echo = with_cancel(
                        &format!("{}-connection, waiting for remote addr", plugin.label()),
                        &shutdown,
                        echo_rx
                            .recv()
                            .map(|opt| opt.ok_or_else(|| anyhow::anyhow!("echo channel closed"))),
                    )
                    .await?;

                    match echo {
                        Echo::Connect(addr) => break addr,
                        other => {
                            echo_tx.send(other).await?;
                        }
                    }
                },
            };

            let (rx, tx): (Box<dyn TransportRead>, Box<dyn TransportWrite>) = {
                match plugin.protocol() {
                    Protocol::TCP => {
                        let stream = TcpStream::connect(remote_addr.to_string()).await?;
                        let (rx, tx) = stream.into_split();
                        (Box::new(TcpRead(rx)), Box::new(TcpWrite(tx)))
                    }
                    Protocol::UDP => {
                        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
                        socket.connect(remote_addr.to_string()).await?;
                        (
                            Box::new(UdpRead(socket.clone())),
                            Box::new(UdpWrite(socket)),
                        )
                    }
                }
            };

            broadcast_tx
                .broadcast(OrderedOutput::new(
                    plugin.label(),
                    Arc::new(vec![HandlerOutput::Messages(vec![Message {
                        msg_type: MsgType::Success,
                        text: format!("Connected to {remote_addr}"),
                    }])]),
                ))
                .await?;

            let local_cancel = shutdown.child_token();

            plugin
                .on_connect(
                    // to send packet into write_task
                    packet_tx.clone(),
                    // to broadcast HandlerOutput to the core plugins
                    broadcast_tx.clone(),
                    &local_cancel,
                    context.clone(),
                )
                .await?;

            let tasks = vec![
                plugin.clone().spawn_read_task(
                    // to send packet into write_task
                    packet_tx.clone(),
                    // to broadcast Vec<HandlerOutput> to the core plugins
                    broadcast_tx.clone(),
                    // to receive packets from server
                    rx,
                    // to receive responses from core plugins
                    echo_rx,
                    // to properly drop connection (local signal)
                    local_cancel.clone(),
                    // shared data
                    context.clone(),
                ),
                plugin.clone().spawn_write_task(
                    // to receive packets from read_task
                    packet_rx,
                    // to broadcast Vec<HandlerOutput> to the core plugins
                    broadcast_tx.clone(),
                    // to send packets to the server
                    tx,
                    // to properly drop connection (local signal)
                    local_cancel.clone(),
                    // shared data
                    context.clone(),
                ),
            ];

            let (first, _, remaining) = select_all(tasks).await;
            local_cancel.cancel();

            // properly finishing the remaining tasks since we use cancellation token
            for future in remaining {
                let _ = future.await;
            }

            match first {
                Ok(Err(err)) => Err(err),
                Err(err) => Err(err.into()),
                _ => Ok(()),
            }
        })
    }

    fn spawn_read_task(
        self: Arc<Self>,
        packet_tx: Sender<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        mut read_half: Box<dyn TransportRead>,
        mut echo_rx: Receiver<Echo>,
        local_cancel: CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> Task {
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];
            let mut packet_buf = Vec::new();
            let mut processors = self.get_processors();
            let mut reader = self.get_reader();

            {
                let mut guard = context.write().await;
                for processor in processors.iter_mut() {
                    processor.init(&mut guard)?;
                }
            }

            loop {
                let next_packet = self.read_next_packet(
                    &mut read_half,
                    &mut buffer,
                    &mut packet_buf,
                    &mut reader,
                    &local_cancel,
                    context.clone(),
                );

                tokio::select! {
                    // We set `biased` to ensure the branches are polled in order.
                    biased;
                    _ = local_cancel.cancelled() => {
                        break;
                    },
                    Some(Echo::Drop) = echo_rx.recv() => {
                        local_cancel.cancel();
                    },
                    result = next_packet => {
                        let mut packet: Packet = result?;

                        let mut outputs: Vec<HandlerOutput> = vec![];
                        for processor in processors.iter_mut() {
                            let result = processor.process(&mut packet, context.clone()).await;
                            match result {
                                Ok(Some(chunk)) => {
                                    for output in chunk.into_iter() {
                                        match output {
                                            HandlerOutput::Requests(mut reqs) => {
                                                self.handle_requests(
                                                    &mut reqs,
                                                    &mut outputs,
                                                    packet_tx.clone(),
                                                    broadcast_tx.clone(),
                                                    &mut echo_rx,
                                                    local_cancel.clone(),
                                                    context.clone(),
                                                ).await?;
                                            },
                                            other => { outputs.push(other); }
                                        }
                                    }

                                    self.handle_outputs(
                                        Arc::new(std::mem::take(&mut outputs)),
                                        packet_tx.clone(),
                                        broadcast_tx.clone(),
                                        &local_cancel,
                                    ).await?;
                                },
                                Ok(None) => {},
                                Err(err) => {
                                    broadcast_tx.broadcast(OrderedOutput::new(
                                        self.label(),
                                        Arc::new(vec![
                                            HandlerOutput::Messages(vec![
                                                Message {
                                                    msg_type: MsgType::Error,
                                                    // TODO: refactor Message to properly show details (or refactor UI)
                                                    text: format!(
                                                        "{}: {}",
                                                        packet.metadata.packet_name,
                                                        err
                                                    ),
                                                },
                                            ]),
                                        ]),
                                    )).await?;

                                    continue;
                                },
                            }
                        }

                        // see tui/dbg_ui plugins for example how we process the packets for output
                        broadcast_tx.broadcast(
                            OrderedOutput::new(self.label(), Arc::new(vec![
                                HandlerOutput::Packets(vec![packet.clone()]),
                            ]))
                        ).await?;
                    },
                }
            }

            broadcast_tx
                .broadcast(OrderedOutput::new(
                    self.label(),
                    Arc::new(vec![HandlerOutput::Messages(vec![Message {
                        msg_type: MsgType::Info,
                        text: format!("Read task for \"{}-connection\" was dropped", self.label()),
                    }])]),
                ))
                .await?;

            Ok(())
        })
    }

    fn spawn_write_task(
        self: Arc<Self>,
        mut packet_rx: Receiver<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        mut write_half: Box<dyn TransportWrite>,
        local_cancel: CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> Task {
        tokio::spawn(async move {
            let mut serializer = self.get_serializer();

            loop {
                tokio::select! {
                    biased;
                    _ = local_cancel.cancelled() => {
                        break;
                    },
                    Some(packets) = packet_rx.recv() => {
                        for packet in packets {
                            let guard = context.read().await;
                            let mut bytes = serializer.serialize(&packet, &guard)?;

                            with_cancel(
                                &format!("{}-connection, write_task", self.label()),
                                &local_cancel,
                                write_half.write(&mut bytes)
                            ).await?;
                        }
                    },
                }
            }

            broadcast_tx
                .broadcast(OrderedOutput::new(
                    self.label(),
                    Arc::new(vec![HandlerOutput::Messages(vec![Message {
                        msg_type: MsgType::Info,
                        text: format!("Write task for \"{}-connection\" was dropped", self.label()),
                    }])]),
                ))
                .await?;

            Ok(())
        })
    }

    async fn read_next_packet(
        &self,
        read_half: &mut Box<dyn TransportRead>,
        buffer: &mut [u8],
        packet_buf: &mut Vec<u8>,
        reader: &mut Box<dyn BytesRead>,
        local_cancel: &CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Packet> {
        loop {
            let guard = context.read().await;
            match reader.read(packet_buf, &guard) {
                Ok(packet) => {
                    packet_buf.drain(..packet.metadata.packet_size);
                    return Ok(packet);
                }
                Err(_) => {
                    // TODO: add errors handling
                    let count = with_cancel(
                        &format!("{}-connection, read_next_packet", self.label()),
                        local_cancel,
                        read_half.read(buffer),
                    )
                    .await?;

                    // TODO: process this error properly we do not need to attempt the read again after it
                    if count == 0 {
                        anyhow::bail!("Connection closed by peer")
                    }

                    packet_buf.extend_from_slice(&buffer[..count]);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_requests(
        &self,
        requests: &mut Vec<Request>,
        outputs: &mut Vec<HandlerOutput>,
        packet_tx: Sender<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        echo_rx: &mut Receiver<Echo>,
        local_cancel: CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<()> {
        let mut index = 0;
        while index < requests.len() {
            match &mut requests[index] {
                Request::InitChoice(_, option) => {
                    if let Some(callback) = option.take() {
                        let chunk = requests.drain(..=index).collect::<Vec<_>>();
                        if !chunk.is_empty() {
                            outputs.push(HandlerOutput::Requests(chunk));
                        }

                        self.handle_outputs(
                            Arc::new(std::mem::take(outputs)),
                            packet_tx.clone(),
                            broadcast_tx.clone(),
                            &local_cancel,
                        )
                        .await?;

                        loop {
                            let echo = with_cancel(
                                &format!("{}-connection, waiting for choice", self.label()),
                                &local_cancel,
                                echo_rx.recv().map(|opt| {
                                    opt.ok_or_else(|| anyhow::anyhow!("echo channel closed"))
                                }),
                            )
                            .await?;

                            // TODO: Some echoes may be lost here; needs investigation.
                            if let Echo::Choose(ids) = echo {
                                outputs.extend(callback(ids)?);
                                break;
                            }
                        }

                        // reset index because requests size was reduced
                        index = 0;
                        continue;
                    }
                }
                Request::SetContext(option) => {
                    if let Some(callback) = option.take() {
                        let mut guard = context.write().await;
                        callback(&mut guard);
                    }
                }
                _ => {}
            }

            index += 1;
        }

        if !requests.is_empty() {
            outputs.push(HandlerOutput::Requests(std::mem::take(requests)));

            self.handle_outputs(
                Arc::new(std::mem::take(outputs)),
                packet_tx.clone(),
                broadcast_tx.clone(),
                &local_cancel,
            )
            .await?;
        }

        Ok(())
    }

    async fn handle_outputs(
        &self,
        outputs: Arc<Vec<HandlerOutput>>,
        packet_tx: Sender<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        local_cancel: &CancellationToken,
    ) -> anyhow::Result<()> {
        if self.enabled_outgoing() {
            broadcast_tx
                .broadcast(OrderedOutput::new(self.label(), outputs.clone()))
                .await?;

            for output in &*outputs {
                if let HandlerOutput::Packets(packets) = output {
                    with_cancel(
                        &format!("{}-connection, handle_outputs", self.label()),
                        local_cancel,
                        packet_tx
                            .send(packets.clone())
                            .map_err(|e| anyhow::anyhow!(e.to_string())),
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn on_connect(
        &self,
        packet_tx: Sender<Vec<Packet>>,
        broadcast_tx: async_broadcast::Sender<OrderedOutput>,
        local_cancel: &CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<()> {
        for mut builder in self.get_builders() {
            let outputs = Arc::new(builder.build(context.clone()).await?);

            self.handle_outputs(
                outputs.clone(),
                packet_tx.clone(),
                broadcast_tx.clone(),
                local_cancel,
            )
            .await?;
        }

        Ok(())
    }

    fn get_builders(&self) -> Vec<Box<dyn OutputBuilder>> {
        vec![]
    }

    fn get_reader(&self) -> Box<dyn BytesRead>;
    fn get_serializer(&self) -> Box<dyn Serializer>;

    fn get_processors(&self) -> Vec<Box<dyn Processor>> {
        let mut processors = vec![];

        for loader in inventory::iter::<PluginLoader<dyn ProcessorPlugin>> {
            let plugin = (loader.load)();
            if plugin.label() == self.label() {
                processors.extend(plugin.get_processors());
            }
        }

        processors
    }

    fn label(&self) -> ServerLabel;
    fn protocol(&self) -> Protocol;
    fn remote_addr(&self) -> anyhow::Result<Option<String>> {
        Ok(None)
    }
    fn enabled_outgoing(&self) -> bool {
        true
    }
}

pub struct OutgoingPolicy<T, const ENABLED: bool>(pub T);
impl<T: Default, const ENABLED: bool> Default for OutgoingPolicy<T, ENABLED> {
    fn default() -> Self {
        Self(T::default())
    }
}

#[async_trait]
impl<T, const ENABLED: bool> NetworkPlugin for OutgoingPolicy<T, ENABLED>
where
    T: NetworkPlugin + Send + Sync + 'static,
{
    fn get_builders(&self) -> Vec<Box<dyn OutputBuilder>> {
        self.0.get_builders()
    }

    fn get_reader(&self) -> Box<dyn BytesRead> {
        self.0.get_reader()
    }

    fn get_serializer(&self) -> Box<dyn Serializer> {
        self.0.get_serializer()
    }

    fn get_processors(&self) -> Vec<Box<dyn Processor>> {
        self.0.get_processors()
    }

    fn label(&self) -> ServerLabel {
        self.0.label()
    }

    fn protocol(&self) -> Protocol {
        self.0.protocol()
    }

    fn remote_addr(&self) -> anyhow::Result<Option<String>> {
        self.0.remote_addr()
    }

    fn enabled_outgoing(&self) -> bool {
        ENABLED
    }
}

// this is the part of the NetworkPlugin
// this plugin allows to add 3rd-party processors to the same label
pub trait ProcessorPlugin: Sync {
    fn get_processors(&self) -> Vec<Box<dyn Processor>>;

    fn label(&self) -> ServerLabel;
}

#[async_trait]
pub trait CorePlugin: Send + Sync
where
    Self: 'static,
{
    fn get_tasks(
        &self,
        broadcast_rx: async_broadcast::Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>>;
}

pub async fn with_cancel<F, T>(
    brief_info: &str,
    local_cancel: &CancellationToken,
    future: F,
) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>> + Send,
{
    tokio::select! {
        biased;
        _ = local_cancel.cancelled() => {
            Err(anyhow::anyhow!("Local cancel: {brief_info}"))
        },
        result = future => result,
    }
}
