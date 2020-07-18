use ethers::providers::Provider;
use ethers::types::Filter;

use std::convert::TryFrom;
use std::error::Error;

use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::try_from("http://localhost:8544")?;

    // let ws = ethers::providers::Ws::connect("wss://kovan.infura.io/ws/v3/...").await?;
    // let provider = Provider::new(ws);

    let filter = Filter::new();

    let mut stream = provider.watch(&filter).await?;

    while let Some(log) = stream.next().await {
        println!("Log: {:#?}", log);
    }

    Ok(())
}
