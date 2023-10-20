use subxt::{OnlineClient, PolkadotConfig};

pub mod runtime {
	include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = OnlineClient::<PolkadotConfig>::new().await?;
    Ok(())
}