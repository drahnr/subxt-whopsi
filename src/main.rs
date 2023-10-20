use subxt::{OnlineClient, PolkadotConfig};

// Don't point at a URL in production:
#[subxt::subxt(runtime_metadata_url="ws://localhost:9944")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = OnlineClient::<PolkadotConfig>::new().await?;
    Ok(())
}