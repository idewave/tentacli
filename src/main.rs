extern crate chrono;
extern crate core;
#[macro_use]
extern crate idewave_packet;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;
extern crate yaml_rust;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use anyhow::{Result as AnyResult};

mod client;
mod config;
mod crypto;
mod errors;
mod ipc;
mod macros;
mod network;
mod parsers;
mod serializers;
mod traits;
mod types;
mod ui;
mod utils;

use crate::client::Client;
use crate::ui::UI;

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();

    let host = env::var("CURRENT_HOST").expect("CURRENT_HOST must be set");
    let port = env::var("CURRENT_PORT").expect("CURRENT_PORT must be set");

    let mut client = Client::new();
    client.connect(&host, u16::from_str(&port)?).await?;
    client.run().await?;

    Ok(())
}
