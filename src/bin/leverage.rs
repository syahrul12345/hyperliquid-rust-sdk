use alloy::signers::local::PrivateKeySigner;
use ethers::signers::{Signer};
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient, InfoClient};
use log::info;

#[tokio::main]
async fn main() {
    // Example assumes you already have a position on ETH so you can update margin
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: PrivateKeySigner = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
        .parse()
        .unwrap();
    let info_client = InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap();

    let address = wallet.address();
    let exchange_client = ExchangeClient::new(None, wallet, Some(BaseUrl::Mainnet), None, None)
        .await
        .unwrap();

    let response = exchange_client
        .update_leverage(5, "ETH", false, None)
        .await
        .unwrap();
    info!("Update leverage response: {response:?}");

    let response = exchange_client
        .update_isolated_margin(1.0, "ETH", None)
        .await
        .unwrap();

    // info!("Update isolated margin response: {response:?}");

    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();

    let user_state = info_client.user_state("0x45d3730A8F811519f3CC310d54FFc4D2142b3773".parse().unwrap()).await.unwrap();
    info!("User state: {user_state:?}");
}
