pub use crate::client::packet::{
    BytesRead,
    CalculateMetadata,
    // Metadata (optional but useful in plugins)
    ExtractMetadata,
    MetadataContext,
    MetadataValue,
    OutputBuilder,
    // Core types
    Packet,
    // Behavior / contracts
    PacketHandler,
    PacketOpcode,
    PacketType,

    Processor,
    Serializer,

    serialize_packet_json,
};

pub use crate::client::packet::fields::NullTerminated;
