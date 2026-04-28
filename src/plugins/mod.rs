//! Built-in plugins.
//!
//! This module contains first-party plugins shipped with Tentacli.
//!
//! - `wow` — World of Warcraft protocol implementations
//! - `core` — Core system plugin (request handling, echo dispatch, shutdown wiring)
//! - `tui` — Terminal UI frontend (feature: `tui`)
//! - `dbg_ui` — Debug UI frontend (feature: `dbg-ui`)
//! - `replay` — Offline packet replay plugin.
//!   Replays packets from TrinityCore/MaNGOS-style `World.log` files
//!   through the normal processing pipeline.
//! - `websocket` — Broadcasts TUI-visible output to WebSocket clients.

pub mod core;
#[cfg(feature = "dbg-ui")]
pub mod dbg_ui;
#[cfg(feature = "replay")]
pub mod replay;
#[cfg(feature = "tui")]
pub mod tui;
#[cfg(feature = "websocket")]
pub mod websocket;
pub mod wow;
