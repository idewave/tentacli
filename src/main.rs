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

use dotenv::dotenv;
use anyhow::{Result as AnyResult};

mod features;
mod primary;

use crate::primary::client::Client;

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();

    Client::new().run().await?;

    Ok(())
}
