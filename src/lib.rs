//! # Tentacli
//!
//! Tentacli is a framework for exploring and interacting with network protocols through a
//! plugin-based, client-side architecture.
//!
//! It runs as a protocol participant, not a sniffer or MITM tool: connections are established
//! directly to servers, packets are framed, parsed, processed, and optionally responded to
//! in real time.
//!
//! ## Core Concepts
//!
//! The system is built around three plugin types:
//!
//! ### NetworkPlugin
//! Defines how a connection is established and how raw bytes are framed into packets and
//! serialized back for transmission.
//!
//! Responsibilities:
//! - Select transport (TCP / UDP)
//! - Establish connections
//! - Frame incoming byte streams into packets (`BytesRead`)
//! - Serialize packets for outgoing writes (`Serializer`)
//!
//! ### ProcessorPlugin
//! Extends a specific network plugin by attaching protocol logic.
//!
//! Responsibilities:
//! - Provide parsers and handlers for specific packet types
//! - Generate outgoing packets and requests
//! - Read from and modify the shared runtime context
//!
//! Processor plugins attach to network plugins by matching `ServerLabel`.
//!
//! ### CorePlugin
//! Acts as the system coordinator.
//!
//! Responsibilities:
//! - Receive processing results and events from all plugins
//! - Route requests and control signals back to network plugins
//! - Implement external interfaces (TUI, debug UI, automation, etc.)
//!
//! ## Plugin Registration
//!
//! Plugins are discovered at runtime using the [`inventory`] crate.
//! To register a plugin, use the `register_plugin!` macro in any module
//! that is linked into the final binary:
//!
//! ```rust,ignore
//! use tentacli::register_plugin;
//! use tentacli::client::{NetworkPlugin, ProcessorPlugin, CorePlugin};
//!
//! register_plugin!(MyNetworkPlugin, dyn NetworkPlugin);
//! register_plugin!(MyProcessors, dyn ProcessorPlugin);
//! register_plugin!(MyCore, dyn CorePlugin);
//! ```
//!
//! ## Shared Context
//!
//! All plugins have access to a shared, typed, dynamic runtime context
//! implemented using `anymap2` and synchronized via `Arc<RwLock<CtxMap>>`.
//! This context is used to store protocol state and coordinate logic
//! across connections and plugins.
//!
//! ## Reference Implementations
//!
//! Real-world plugin implementations can be found in the repository:
//! - WoW WotLK network plugin: `plugins/wow/wotlk/login`
//! - Processor plugins: `plugins/wow/wotlk/login/*` and `plugins/wow/wotlk/realm/*`
//! - Core plugins: `plugins/tui`, `plugins/dbg_ui`, `plugins/core`
//!
//! These serve as full reference implementations for connection handling,
//! packet framing, protocol logic, context usage, and UI integration.
//!
//! [`inventory`]: https://docs.rs/inventory

pub mod client;
pub mod plugins;

#[doc(hidden)]
pub use inventory as __inventory;

pub use client::Client;
