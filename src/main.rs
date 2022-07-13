extern crate core;
extern crate yaml_rust;

use crate::client::Client;

mod client;
mod config;
mod crypto;
mod data_storage;
mod network;
mod traits;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.connect("127.0.0.1", 3724).await;
    client.handle_connection().await;
}
