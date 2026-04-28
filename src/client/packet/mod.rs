use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod fields;
pub mod helpers;
pub mod prelude;

use crate::client::types::{CtxMap, HandlerOutput};

#[non_exhaustive]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum PacketOpcode {
    #[default]
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Text(String),
    Raw(Vec<u8>),
}

impl From<u8> for PacketOpcode {
    fn from(v: u8) -> Self {
        Self::U8(v)
    }
}
impl From<u16> for PacketOpcode {
    fn from(v: u16) -> Self {
        Self::U16(v)
    }
}
impl From<u32> for PacketOpcode {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}
impl From<u64> for PacketOpcode {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}
impl From<&str> for PacketOpcode {
    fn from(v: &str) -> Self {
        Self::Text(v.to_owned())
    }
}
impl From<String> for PacketOpcode {
    fn from(v: String) -> Self {
        Self::Text(v)
    }
}
impl From<Vec<u8>> for PacketOpcode {
    fn from(v: Vec<u8>) -> Self {
        Self::Raw(v)
    }
}

macro_rules! impl_try_from_ref {
    ($t:ty, $variant:ident) => {
        impl TryFrom<&PacketOpcode> for $t {
            type Error = anyhow::Error;
            fn try_from(op: &PacketOpcode) -> anyhow::Result<$t> {
                match op {
                    PacketOpcode::$variant(x) => Ok(*x),
                    _ => Err(anyhow::anyhow!("Expected {}, got {:?}", stringify!($t), op)),
                }
            }
        }
    };
}

impl_try_from_ref!(u8, U8);
impl_try_from_ref!(u16, U16);
impl_try_from_ref!(u32, U32);
impl_try_from_ref!(u64, U64);

impl<'a> TryFrom<&'a PacketOpcode> for &'a str {
    type Error = anyhow::Error;
    fn try_from(op: &'a PacketOpcode) -> anyhow::Result<&'a str> {
        match op {
            PacketOpcode::Text(s) => Ok(s.as_str()),
            _ => Err(anyhow::anyhow!("Expected Text, got {:?}", op)),
        }
    }
}

impl<'a> TryFrom<&'a PacketOpcode> for &'a [u8] {
    type Error = anyhow::Error;
    fn try_from(op: &'a PacketOpcode) -> anyhow::Result<&'a [u8]> {
        match op {
            PacketOpcode::Raw(b) => Ok(&b[..]),
            _ => Err(anyhow::anyhow!("Expected Raw, got {:?}", op)),
        }
    }
}

macro_rules! impl_try_from_owned {
    ($t:ty) => {
        impl TryFrom<PacketOpcode> for $t {
            type Error = anyhow::Error;
            fn try_from(op: PacketOpcode) -> anyhow::Result<$t> {
                (&op).try_into()
            }
        }
    };
}

impl_try_from_owned!(u8);
impl_try_from_owned!(u16);
impl_try_from_owned!(u32);
impl_try_from_owned!(u64);

impl TryFrom<PacketOpcode> for String {
    type Error = anyhow::Error;
    fn try_from(op: PacketOpcode) -> anyhow::Result<String> {
        match op {
            PacketOpcode::Text(s) => Ok(s),
            _ => Err(anyhow::anyhow!("Expected Text, got {:?}", op)),
        }
    }
}

impl TryFrom<PacketOpcode> for Vec<u8> {
    type Error = anyhow::Error;
    fn try_from(op: PacketOpcode) -> anyhow::Result<Vec<u8>> {
        match op {
            PacketOpcode::Raw(b) => Ok(b),
            _ => Err(anyhow::anyhow!("Expected Raw, got {:?}", op)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PacketType {
    #[default]
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataValue {
    pub size: usize,
    pub offset: usize,
}

#[derive(Default, Debug)]
pub struct MetadataContext {
    pub offset: usize,
    pub metadata: HashMap<String, MetadataValue>,
    pub current_key: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PacketMetadata {
    pub packet_type: PacketType,
    pub opcode: PacketOpcode,
    pub offsets_info: HashMap<String, MetadataValue>,
    pub packet_name: String,
    pub packet_size: usize,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PacketContent {
    pub json: String,
    pub body: Vec<u8>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Packet {
    pub metadata: PacketMetadata,
    pub content: PacketContent,
}

impl Packet {
    pub fn set_opcode(&mut self, opcode: PacketOpcode) {
        self.metadata.opcode = opcode;
    }

    pub fn set_type(&mut self, packet_type: PacketType) {
        self.metadata.packet_type = packet_type;
    }

    pub fn set_offset_info(&mut self, offsets_info: HashMap<String, MetadataValue>) {
        self.metadata.offsets_info = offsets_info;
    }

    pub fn set_packet_name(&mut self, packet_name: String) {
        self.metadata.packet_name = packet_name;
    }

    pub fn set_packet_size(&mut self, packet_size: usize) {
        self.metadata.packet_size = packet_size;
    }

    pub fn set_json(&mut self, json: String) {
        self.content.json = json;
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.content.body = body;
    }
}

pub fn serialize_packet_json<T: Serialize>(value: &T) -> anyhow::Result<String> {
    let json_value = serde_json::to_value(value)?;
    Ok(serde_json::to_string(&json_value)?)
}

pub trait CalculateMetadata {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext;
}

macro_rules! impl_for_primitive {
    ($($t:ty),*) => {
        $(
            impl CalculateMetadata for $t {
                fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
                    let size = std::mem::size_of::<Self>();
                    ctx.metadata.insert(
                        ctx.current_key.clone(),
                        MetadataValue {
                            size,
                            offset: ctx.offset,
                        },
                    );
                    ctx.offset += size;

                    ctx
                }
            }
        )*
    };
}

impl_for_primitive!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize, bool, char
);

fn join_key(prefix: &str, seg: impl ToString) -> String {
    if prefix.is_empty() {
        seg.to_string()
    } else {
        format!("{}/{}", prefix, seg.to_string())
    }
}

impl<T: CalculateMetadata> CalculateMetadata for Vec<T> {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext {
        for (i, item) in self.iter().enumerate() {
            let prev_key = context.current_key.clone();
            context.current_key = join_key(&prev_key, i);
            item.calculate(context);
            context.current_key = prev_key;
        }

        context
    }
}

impl<T: CalculateMetadata, const N: usize> CalculateMetadata for [T; N] {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext {
        for (i, item) in self.iter().enumerate() {
            let prev_key = context.current_key.clone();
            context.current_key = join_key(&prev_key, i);
            item.calculate(context);
            context.current_key = prev_key;
        }

        context
    }
}

impl<T: CalculateMetadata> CalculateMetadata for Option<T> {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext {
        if let Some(inner) = self {
            inner.calculate(context);
        }

        context
    }
}

impl CalculateMetadata for String {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = self.len();
        context.metadata.insert(
            context.current_key.clone(),
            MetadataValue {
                size,
                offset: context.offset,
            },
        );
        context.offset += size;

        context
    }
}

impl CalculateMetadata for () {
    fn calculate<'a>(&self, context: &'a mut MetadataContext) -> &'a mut MetadataContext {
        context
    }
}

pub trait ExtractMetadata: CalculateMetadata {
    fn extract_metadata(&self) -> HashMap<String, MetadataValue> {
        let mut context = MetadataContext::default();
        std::mem::take(&mut self.calculate(&mut context).metadata)
    }
}

#[async_trait]
pub trait PacketHandler: Send + Sync {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>>;
}

#[async_trait]
pub trait OutputBuilder: Send + Sync {
    async fn build(&mut self, context: Arc<RwLock<CtxMap>>) -> anyhow::Result<Vec<HandlerOutput>>;
}

#[async_trait]
pub trait Processor: Send + Sync {
    fn init(&mut self, _context: &mut CtxMap) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        context: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>>;

    async fn process(
        &mut self,
        packet: &mut Packet,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Option<Vec<HandlerOutput>>> {
        let guard = context.read().await;
        let mut handlers = self.get_handlers(&packet.metadata.opcode, &guard)?;
        self.call_handlers(&mut handlers, packet, context.clone())
            .await
    }

    async fn call_handlers(
        &self,
        handlers: &mut Vec<Box<dyn PacketHandler>>,
        packet: &mut Packet,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Option<Vec<HandlerOutput>>> {
        let mut all_outputs = Vec::new();

        for handler in handlers {
            let outputs = handler.handle(packet, context.clone()).await?;
            all_outputs.extend(outputs);
        }

        Ok(if all_outputs.is_empty() {
            None
        } else {
            Some(all_outputs)
        })
    }
}

pub trait BytesRead: Send + Sync {
    fn read(&mut self, buffer: &mut [u8], context: &CtxMap) -> anyhow::Result<Packet>;
}

pub trait Serializer: Send + Sync {
    fn serialize(&mut self, packet: &Packet, context: &CtxMap) -> anyhow::Result<Vec<u8>>;
}
