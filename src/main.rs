extern crate core;
extern crate yaml_rust;

use crate::client::Client;
use crate::ui::UI;

mod client;
mod config;
mod crypto;
mod data_storage;
mod logger;
mod network;
mod traits;
mod types;
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.connect("127.0.0.1", 3724).await.unwrap();
    client.handle_connection().await;
}
