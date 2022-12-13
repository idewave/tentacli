extern crate chrono;
extern crate core;
#[macro_use]
extern crate idewave_packet;
#[macro_use]
extern crate serde;
extern crate yaml_rust;

use dotenv::dotenv;
use std::env;

use crate::client::Client;
use crate::ui::UI;

mod client;
mod config;
mod crypto;
mod ipc;
mod macros;
mod network;
mod parsers;
mod serializers;
mod traits;
mod types;
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("CURRENT_HOST").expect("CURRENT_HOST must be set");

    let mut client = Client::new();
    client.connect(&host, 3724).await.unwrap();
    client.run().await;
}
