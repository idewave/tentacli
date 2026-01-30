pub use crate::client::packet::{
    // Behavior / contracts
    PacketHandler,
    OutputBuilder,
    Processor,
    BytesRead,
    Serializer,

    // Core types
    Packet,
    PacketOpcode,
    PacketType,

    // Metadata (optional but useful in plugins)
    ExtractMetadata,
    CalculateMetadata,
    MetadataValue,
    MetadataContext,
    serialize_packet_json,
};

pub use crate::client::packet::fields::{
    NullTerminated
};