use algonaut::core::Round;
use algonaut::Algod;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = Algod::new()
        .bind(env::var("ALGOD_URL")?.as_ref())
        .auth(env::var("ALGOD_TOKEN")?.as_ref())
        .client_v2()?;

    // print algod status
    let node_status = algod.status().await?;
    println!("algod last round: {}", node_status.last_round);
    println!(
        "algod time since last round: {}",
        node_status.time_since_last_round
    );
    println!("algod catchup: {}", node_status.catchup_time);
    println!("algod latest version: {}", node_status.last_version);

    // fetch block information
    let last_block = algod.block(Round(node_status.last_round)).await?;
    println!("{:#?}", last_block);

    Ok(())
}
