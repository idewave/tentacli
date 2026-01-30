//! Built-in plugins.
//!
//! This module contains first-party plugins shipped with Tentacli.
//!
//! - `wow` — World of Warcraft protocol implementations
//! - `core` — Core system plugin (request handling, echo dispatch, shutdown wiring)
//! - `tui` — Terminal UI frontend (feature: `tui`)
//! - `dbg_ui` — Debug UI frontend (feature: `dbg-ui`)

pub mod wow;
#[cfg(feature = "tui")]
pub mod tui;
pub mod core;
#[cfg(feature = "dbg-ui")]
pub mod dbg_ui;