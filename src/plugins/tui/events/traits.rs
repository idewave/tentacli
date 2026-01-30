use async_event_emitter::AsyncEventEmitter;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::client::Echo;

pub trait EventType: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {}
impl<T> EventType for T where T: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {}

pub struct EventReceiver(Box<dyn Any + Send + Sync>);
impl EventReceiver {
    pub fn new<T: EventType>(rx: Receiver<T>) -> Self {
        Self(Box::new(rx))
    }

    pub fn downcast_mut<T: EventType>(&mut self) -> Option<&mut Receiver<T>> {
        self.0.downcast_mut::<Receiver<T>>()
    }
}

pub struct EventSender(Box<dyn Any + Send + Sync>);
impl EventSender {
    pub fn new<T: EventType>(tx: Sender<T>) -> Self {
        Self(Box::new(tx))
    }

    pub fn downcast_ref<T: EventType>(&self) -> Option<&Sender<T>> {
        self.0.downcast_ref::<Sender<T>>()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EmitType {
    Local,
    Parent,
}

#[derive(Default)]
pub struct EventSystem {
    pub parent_emitter: Arc<AsyncEventEmitter>,
    pub local_emitter: Arc<AsyncEventEmitter>,
    receivers: HashMap<(TypeId, EmitType), EventReceiver>,
    senders: HashMap<(TypeId, EmitType), EventSender>,
}

impl EventSystem {
    pub fn new(parent_emitter: Arc<AsyncEventEmitter>) -> Self {
        Self {
            parent_emitter,
            local_emitter: Arc::new(AsyncEventEmitter::default()),
            receivers: HashMap::new(),
            senders: HashMap::new(),
        }
    }

    pub async fn emit<T: EventType>(&self, event: T, emit_type: EmitType) -> anyhow::Result<()> {
        let name = std::any::type_name::<T>();
        match emit_type {
            EmitType::Local => self.local_emitter.emit(name, event).await?,
            EmitType::Parent => self.parent_emitter.emit(name, event).await?,
        }
        Ok(())
    }

    pub fn receiver<T: EventType>(&mut self, emit_type: EmitType) -> Option<&mut Receiver<T>> {
        self.receivers
            .get_mut(&(TypeId::of::<T>(), emit_type))
            .and_then(|r| r.downcast_mut())
    }

    pub fn sender<T: EventType>(&self, emit_type: EmitType) -> Option<Sender<T>> {
        self.senders
            .get(&(TypeId::of::<T>(), emit_type))
            .and_then(|s| s.downcast_ref())
            .cloned()
    }

    pub fn add_channel<T: EventType>(
        &mut self,
        emit_type: EmitType,
        sender: Sender<T>,
        receiver: Receiver<T>,
    ) {
        let key = (TypeId::of::<T>(), emit_type);
        self.senders.insert(key, EventSender::new(sender));
        self.receivers.insert(key, EventReceiver::new(receiver));
    }
}

pub trait WithEventSystem {
    fn event_system(&mut self) -> &mut EventSystem;

    fn bind_event_system(&mut self, emitter: Arc<AsyncEventEmitter>) {
        *self.event_system() = EventSystem::new(emitter);
    }
}

pub trait EventsRuntime: WithEventSystem {
    async fn register_all(&mut self, emitter: Arc<AsyncEventEmitter>) -> anyhow::Result<()>;

    async fn register_as_child(&mut self, emitter: Arc<AsyncEventEmitter>) -> anyhow::Result<()> {
        self.bind_event_system(emitter.clone());
        self.register_all(emitter).await
    }

    async fn try_update_all(&mut self) -> anyhow::Result<Vec<Echo>>;

    async fn emit_down<T: EventType>(&mut self, event: T) -> anyhow::Result<()> {
        self.event_system().emit(event, EmitType::Local).await
    }

    async fn emit_up<T: EventType>(&mut self, event: T) -> anyhow::Result<()> {
        self.event_system().emit(event, EmitType::Parent).await
    }
}

pub trait EventHandler<T: EventType>: WithEventSystem {
    const CHANNEL_CAPACITY: usize = 50;

    async fn callback(&mut self, event: T) -> anyhow::Result<Vec<Echo>>;

    fn create_channel(&mut self) -> (Sender<T>, Receiver<T>) {
        tokio::sync::mpsc::channel(Self::CHANNEL_CAPACITY)
    }

    async fn subscribe(
        &mut self,
        emitter: Arc<AsyncEventEmitter>,
        emit_type: EmitType,
    ) -> anyhow::Result<()> {
        let Some(sender) = self.event_system().sender::<T>(emit_type) else {
            anyhow::bail!(
                "Sender for {}::{:?} not found",
                std::any::type_name::<T>(),
                emit_type
            )
        };

        emitter.on(std::any::type_name::<T>(), move |event| {
            let sender = sender.clone();
            async move {
                let _ = sender.send(event).await;
            }
        });

        Ok(())
    }

    async fn register_event(&mut self, emit_type: EmitType) -> anyhow::Result<()> {
        let (sender, receiver) = self.create_channel();
        self.event_system().add_channel::<T>(emit_type, sender, receiver);

        let emitter = match emit_type {
            EmitType::Local => self.event_system().local_emitter.clone(),
            EmitType::Parent => self.event_system().parent_emitter.clone(),
        };

        self.subscribe(emitter, emit_type).await
    }

    async fn try_update(&mut self, emit_type: EmitType) -> anyhow::Result<Vec<Echo>> {
        let mut events = Vec::new();

        if let Some(receiver) = self.event_system().receiver::<T>(emit_type) {
            while let Ok(event) = receiver.try_recv() {
                events.push(event);
            }
        }

        let mut output = Vec::new();
        for event in events {
            output.extend(self.callback(event).await?);
        }

        Ok(output)
    }
}