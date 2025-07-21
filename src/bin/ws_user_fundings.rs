use log::info;

use std::str::FromStr;

use alloy::primitives::Address;
use hyperliquid_rust_sdk::{BaseUrl, InfoClient, Message, Subscription};
use tokio::{
    spawn,
    sync::mpsc::unbounded_channel,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();
    let user = Address::from_str("0x6FD45EE91654730b67c4E6e67804cDEC31EcF38d").unwrap();

    let (sender, mut receiver) = unbounded_channel();
    let subscription_id = info_client
        .subscribe(Subscription::UserFundings { user }, sender)
        .await
        .unwrap();

    // spawn(async move {
    //     sleep(Duration::from_secs(30)).await;
    //     info!("Unsubscribing from user fundings data");
    //     info_client.unsubscribe(subscription_id).await.unwrap()
    // });

    // this loop ends when we unsubscribe
    while let Some(Message::UserFundings(user_fundings)) = receiver.recv().await {
        info!("Received user fundings data: {user_fundings:?}");
    }
}
