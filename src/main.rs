use subxt::{OnlineClient, PolkadotConfig};

pub mod runtime {
	include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = OnlineClient::<PolkadotConfig>::new().await?;
    Ok(())
}