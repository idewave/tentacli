use anymap2::Map;
use async_broadcast::RecvError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::task::JoinHandle;

use crate::client::packet::Packet;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MsgType {
    Success,
    Error,
    #[default]
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub msg_type: MsgType,
    pub text: String,
}

pub type Task = JoinHandle<anyhow::Result<()>>;
pub type ServerLabel = &'static str;

pub type Callback =
    Box<dyn FnOnce(Vec<usize>) -> anyhow::Result<Vec<HandlerOutput>> + Send + Sync + 'static>;

pub type CtxMap = Map<dyn anymap2::any::Any + Send + Sync>;
pub type ContextCallback = Box<dyn FnOnce(&mut CtxMap) + Sync + Send>;

#[derive(Clone)]
pub struct ChoiceItems(pub Arc<Vec<String>>);

impl ChoiceItems {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.0.iter()
    }

    pub fn get(&self, idx: usize) -> Option<&String> {
        self.0.get(idx)
    }
}

impl Serialize for ChoiceItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ChoiceItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<String>::deserialize(deserializer)?;
        Ok(ChoiceItems(Arc::new(vec)))
    }
}

#[non_exhaustive]
pub enum Request {
    Drop(ServerLabel),
    Connect(ServerLabel, String),
    InitChoice(ChoiceItems, Option<Callback>),
    SetContext(Option<ContextCallback>),
}

#[non_exhaustive]
#[derive(Clone)]
pub enum Echo {
    Choose(Vec<usize>),
    Connect(String),
    Drop,
}

#[non_exhaustive]
pub enum HandlerOutput {
    Requests(Vec<Request>),
    Packets(Vec<Packet>),
    Messages(Vec<Message>),
}

static HO_SEQ: AtomicU64 = AtomicU64::new(0);

#[inline]
fn next_seq() -> u64 {
    HO_SEQ.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct OrderedOutput {
    seq: u64,
    pub label: ServerLabel,
    pub outputs: Arc<Vec<HandlerOutput>>,
}

impl OrderedOutput {
    pub fn new(label: ServerLabel, outputs: Arc<Vec<HandlerOutput>>) -> Self {
        Self {
            seq: next_seq(),
            label,
            outputs,
        }
    }
}

pub struct OrderedReceiver {
    broadcast_rx: async_broadcast::Receiver<OrderedOutput>,
    buffer: BTreeMap<u64, OrderedOutput>,
    next: Option<u64>,
}

impl OrderedReceiver {
    pub fn new(broadcast_rx: async_broadcast::Receiver<OrderedOutput>) -> Self {
        Self {
            broadcast_rx,
            buffer: BTreeMap::new(),
            next: None,
        }
    }

    pub async fn recv(&mut self) -> Result<OrderedOutput, RecvError> {
        loop {
            let msg = self.broadcast_rx.recv().await?;

            let seq = msg.seq;
            self.buffer.insert(seq, msg);

            if self.next.is_none() {
                self.next = self.buffer.keys().next().copied();
            }

            if let Some(next) = self.next {
                if let Some(entry) = self.buffer.remove(&next) {
                    self.next = Some(next + 1);
                    return Ok(entry);
                }
            }

            // soft-fail protection
            const MAX: usize = 256;
            if self.buffer.len() > MAX {
                self.next = self.buffer.keys().next().copied();
            }
        }
    }
}
