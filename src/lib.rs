//! TentaCLI is embeddable, extendable console client for WoW 3.3.5a server.
//!
//! You can use it directly by compiling with cargo build,
//! or you can incorporate it as a library in your own application.
//! Also you can implement own feature set and pass it to the `run()` method.
//! See `Feature` trait and `RunOptions`.
//!
//! What this client can do:
//! - it can parse basic packet set, such as SMSG_MESSAGECHAT or SMSG_UPDATE_OBJECT
//! - it allows you to login on any server, but you can enter the world only on servers without Warden anti-cheat
//! - you can use `autoselect` options in config file to set default Realm/Character and avoid the step of selecting this data manually
//! - if installed with `ui` feature (installed by default), it allows scrolling the packets history using keyboard and seeing the details for each packet
//! - if installed with `console` feature, it will display only minimal output
//! - if installed without any feature, client will output nothing (but you still can provide own output feature)
//! - you can implement own packet processors and send them using custom features
//! - you can pass external data storage to the tentacli using **CreateOptions**
//!
//! ## Examples
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use anyhow::{Result as AnyResult};
//!
//! use tentacli::async_broadcast::{BroadcastSender, BroadcastReceiver};
//! use tentacli::{Client, CreateOptions, RunOptions};
//! use tentacli_traits::{Feature, FeatureError};
//! use tentacli_traits::types::{HandlerOutput, ProcessorFunction, ProcessorResult};
//!
//! #[tokio::main]
//! async fn main() {
//!     use tentacli_traits::types::Task;
//! #[derive(Default)]
//!     pub struct MyFeature {
//!         _receiver: Option<BroadcastReceiver<HandlerOutput>>,
//!         _sender: Option<BroadcastSender<HandlerOutput>>,
//!     }
//!
//!     impl Feature for MyFeature {
//!         fn set_broadcast_channel(
//!             &mut self,
//!             sender: BroadcastSender<HandlerOutput>,
//!             receiver: BroadcastReceiver<HandlerOutput>
//!         ) {
//!             self._sender = Some(sender);
//!             self._receiver = Some(receiver);
//!         }
//!
//!         fn get_tasks(&mut self) -> anyhow::Result<Vec<Task>> {
//!             let mut receiver = self._receiver.as_mut().ok_or(FeatureError::ReceiverNotFound)?.clone();
//!
//!             let handle_smth = || {
//!                 tokio::spawn(async move {
//!                     loop {
//!                         if let Ok(output) = receiver.recv().await {
//!                             match output {
//!                                 HandlerOutput::SuccessMessage(message, _) => {
//!                                     println!("{}", message);
//!                                 }
//!                                 _ => {}
//!                             }
//!                         }
//!                     }
//!                 })
//!             };
//!
//!             Ok(vec![handle_smth()])
//!         }
//!
//!         fn get_login_processors(&self) -> Vec<ProcessorFunction> {
//!             vec![]
//!         }
//!
//!         fn get_realm_processors(&self) -> Vec<ProcessorFunction> {
//!             vec![]
//!         }
//!
//!         fn get_one_time_handler_maps(&self) -> Vec<BTreeMap<u16, ProcessorResult>> {
//!             vec![]
//!         }
//!
//!         fn get_initial_processors(&self) -> Vec<ProcessorFunction> {
//!             vec![]
//!         }
//!     }
//!
//!     let options = RunOptions {
//!         external_features: vec![Box::new(MyFeature::default())],
//!         account: "account_name",
//!         config_path: "./dir/another_dir/ConfigFileName.yml",
//!         dotenv_path: "./path/to/.env"
//!     };
//!
//!     // ... pass options to the client
//!     // Client::new(CreateOptions::default()).run(options).await.unwrap();
//! }
//! ```

#[macro_use]
extern crate cfg_if;
#[cfg(feature = "ui")]
extern crate chrono;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tentacli_packet;

pub use primary::client::{Client, CreateOptions, RunOptions};

mod features;
mod primary;

pub mod async_broadcast {
    pub use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
}

pub mod serializers {
    pub use crate::primary::serializers::serialize_array;
}