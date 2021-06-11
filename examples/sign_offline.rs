use algonaut::core::MicroAlgos;
use algonaut::crypto::mnemonic;
use algonaut::transaction::account::Account;
use algonaut::transaction::{Pay, Txn};
use algonaut::Algod;
use algonaut_transaction::ApiSignedTransaction;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

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
    println!("node_status: {:?}", node_status);

    let account = Account::generate();
    println!("Public Key: {:?}", account.address().to_string());

    let m = mnemonic::from_key(&account.seed())?;
    println!("Backup phrase: {}", m);

    let params = algod.transaction_params().await?;

    let t = Txn::new()
        .sender(account.address())
        .first_valid(params.last_round)
        .last_valid(params.last_round + 1000)
        .genesis_id(params.genesis_id)
        .genesis_hash(params.genesis_hash)
        .fee(MicroAlgos(10_000))
        .payment(
            Pay::new()
                .amount(MicroAlgos(123_456))
                .to("4MYUHDWHWXAKA5KA7U5PEN646VYUANBFXVJNONBK3TIMHEMWMD4UBOJBI4".parse()?)
                .build(),
        )
        .build();

    println!("Made unsigned transaction: {:?}", t);

    // sign the transaction
    let signed_transaction = account.sign_and_generate_signed_transaction(&t)?;
    let bytes = rmp_serde::to_vec_named(&ApiSignedTransaction::from(signed_transaction))?;

    let filename = "./signed.tx";
    let mut f = File::create(filename)?;
    f.write_all(&bytes)?;

    println!("Saved signed transaction to file: {}", filename);

    Ok(())
}
