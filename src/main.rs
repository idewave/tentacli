use anyhow::{Result as AnyResult};

use tentacli::{Client, RunOptions};

#[tokio::main]
async fn main() -> AnyResult<()> {
    Client::new().run(RunOptions {
        external_channel: None,
        external_features: vec![],
        account: "bot",
        config_path: "./conf/Config.yml",
        dotenv_path: "./conf/.env"
    }).await?;

    Ok(())
}
