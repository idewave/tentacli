use anyhow::{Result as AnyResult};

use tentacli::{Client, RunOptions};

#[tokio::main]
async fn main() -> AnyResult<()> {
    Client::new().run(RunOptions {
        external_features: vec![],
        account: "bot1",
        config_path: "Config.yml",
        dotenv_path: ".env"
    }).await?;

    Ok(())
}
