use dotenv::dotenv;
use anyhow::{Result as AnyResult};

use tentacli::{Client};

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();

    Client::new().run(None).await?;

    Ok(())
}
