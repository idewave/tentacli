use cfg_if::cfg_if;

use tentacli::{Client, CreateOptions, RunOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cfg_if! {
        if #[cfg(feature = "debug")] {
            console_subscriber::init();
        }
    }

    Client::new(CreateOptions::default()).run(RunOptions {
        external_features: vec![],
        account: "bot1",
        config_path: "Config.yml",
        dotenv_path: ".env",
    }).await?;

    Ok(())
}
