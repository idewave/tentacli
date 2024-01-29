use dotenv::dotenv;
use anyhow::{Result as AnyResult};

use tentacli::{Client, RunOptions};

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();

    Client::new().run(RunOptions {
        external_channel: None,
        external_features: vec![],
    }).await?;

    Ok(())
}
