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
//! - it can parse basic packet set, such as SMSG_MESSAGECHAT or SMSG_UPDATE_OBJECT
//! - it allows you to login on any server, but you can enter the world only on servers without Warden anti-cheat
//! - you can use `autoselect` options in config file to set default Realm/Character and avoid the step of selecting
//! this data manually
//! - if installed with `ui` feature (installed by default), it allows scrolling the packets history using keyboard and
//! seeing the details for each packet
//! - if installed with `console` feature, it will display only minimal output
//! - if installed without any feature, client will output nothing (but you still can provide own output feature)
//!
//! ## Examples
//!
//! ```rust
//! use tokio::task::JoinHandle;
//!
//! use tentacli::async_broadcast::{broadcast, BroadcastSender, BroadcastReceiver};
//! use tentacli::{Client, RunOptions};
//! use tentacli::traits::Feature;
//! use tentacli::types::HandlerOutput;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     let (query_sender, query_receiver) = broadcast::<HandlerOutput>(100);
//!
//!     pub struct MyFeature {
//!         _receiver: BroadcastReceiver<HandlerOutput>,
//!         _sender: BroadcastSender<HandlerOutput>,
//!     }
//!
//!     impl Feature for MyFeature {
//!         fn new(
//!             sender: BroadcastSender<HandlerOutput>,
//!             receiver: BroadcastReceiver<HandlerOutput>
//!         ) -> Self where Self: Sized {
//!             Self {
//!                 _receiver: receiver,
//!                 _sender: sender,
//!             }
//!         }
//!
//!         fn get_tasks(&mut self) -> Vec<JoinHandle<()>> {
//!             let mut receiver = self._receiver.clone();
//!
//!             let handle_smth = || {
//!                 tokio::spawn(async move {
//!                     loop {
//!                         if let Ok(output) = receiver.recv().await {
//!                             match output {
//!                                 HandlerOutput::SuccessMessage(message, _) => {
//!                                     println!("{}", message);
//!                                 },
//!                                 _ => {},
//!                             }
//!                         }
//!                     }
//!                 })
//!             };
//!
//!             vec![handle_smth()]
//!         }
//!     }
//!
//!     let options = RunOptions {
//!         external_channel: Some((query_sender.clone(), query_receiver.clone())),
//!         external_features: vec![Box::new(MyFeature::new(query_sender, query_receiver))],
//!         account: "account_name",
//!         config_path: "./dir/another_dir/ConfigFileName.yml",
//!         dotenv_path: "./path/to/.env"
//!     };
//!
//!     // ... pass options to the client
//!     // Client::new().run(options).await.unwrap();
//! }
//! ```

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

pub mod async_broadcast {
    pub use async_broadcast::{broadcast, Sender as BroadcastSender, Receiver as BroadcastReceiver};
}

pub mod chat {
    pub use crate::primary::client::chat::types::{Language, MessageType, TextEmoteType, EmoteType};
}

pub mod movement {
    pub use crate::primary::client::movement::types::{MovementFlags, MovementFlagsExtra};
}

pub mod player {
    pub use crate::primary::client::{
        ObjectField, Player, PlayerField, Position, UnitField, Race, Class, Gender
    };
}

pub mod realm {
    pub use crate::primary::client::{Realm};
}

pub mod packet {
    pub mod custom_fields {
        pub use crate::primary::types::PackedGuid;
        pub use crate::primary::types::TerminatedString;
    }

    pub mod chat {
        pub use crate::primary::client::chat::packet::{ChatOutcome, EmoteOutcome, TextEmoteOutcome};
    }
    
    pub mod movement {
        pub use crate::primary::client::movement::packet::{MovementOutcome, MovementOpcodes};
    }

    pub mod player {
        pub use crate::primary::client::player::packet::CharCreateOutcome;
    }
}

pub mod traits {
    pub use crate::primary::traits::Feature;
    pub use crate::primary::traits::binary_converter::BinaryConverter;
}

pub mod types {
    pub use crate::primary::types::HandlerOutput;
}