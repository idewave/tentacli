use tentacli::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    if let Some(cmd) = args.next()
        && cmd == "doctor"
    {
        Client::doctor()?;
        return Ok(());
    }

    Client::run(None).await
}
