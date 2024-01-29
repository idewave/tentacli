//! TentaCLI is embeddable, extendable console client for WoW 3.3.5a server.
//!
//! You can use it directly by compiling with cargo build,
//! or you can incorporate it as a library in your own application.
//! TentaCLI accepts external broadcast channel (`async_broadcast` crate),
//! so you can connect it with the rest of your application.
//! Also you can implement own feature set and pass it to the `run()` method.
//! See `Feature` trait and `RunOptions`.
//!
//! What this client can do:
//! - it can parse basic packet set, like SMSG_MESSAGECHAT or SMSG_UPDATE_OBJECT
//! - it allows you to login on any server, but you can enter the world only on servers without Warden anti-cheat
//! - you can use `autoselect` options in Config.yml to set default Realm/Character and avoid the step of selecting
//! this data manually
//! - if installed with `ui` feature (installed by default), it allows scrolling the packets history using keyboard and
//! seeing the details for each packet
//! - if installed with `console` feature, it will displays only minimal output
//! - if installed without any feature, client will output nothing (but you still can provide own output feature)
//!

extern crate chrono;
#[macro_use]
extern crate idewave_packet;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;
extern crate yaml_rust;
#[macro_use]
extern crate cfg_if;

mod features;
mod primary;

pub use primary::client::{Client, RunOptions};
pub use primary::traits::Feature;
pub use primary::types::HandlerOutput;