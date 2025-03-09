use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "135a1fd962e917e703b11e6393ec457c87a9b1b514537be76737ebd86e28c9b5"
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(421614 as u64);
    let address = wallet.address();
    println!("address: {:?}", address);
    let exchange_client =
        ExchangeClient::new(None, wallet.clone(), Some(BaseUrl::Mainnet), None, None)
            .await
            .unwrap();

    let max_fee_rate = "0.001%";
    let builder = "0xDF06e2472784fffA3FFED9Ed4a05425DC569D24a"
        .parse::<Address>()
        .unwrap();

    let resp = exchange_client
        .approve_builder_fee(builder, max_fee_rate.to_string(), Some(&wallet))
        .await;
    info!("resp: {resp:#?}");
}
