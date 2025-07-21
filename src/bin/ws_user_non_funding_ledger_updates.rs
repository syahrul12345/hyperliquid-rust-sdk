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
    let user = Address::from_str("0x6fd45ee91654730b67c4e6e67804cdec31ecf38d").unwrap();

    let (sender, mut receiver) = unbounded_channel();
    let subscription_id = info_client
        .subscribe(Subscription::UserNonFundingLedgerUpdates { user }, sender)
        .await
        .unwrap();

    // spawn(async move {
    //     sleep(Duration::from_secs(30)).await;
    //     info!("Unsubscribing from user non funding ledger update data");
    //     info_client.unsubscribe(subscription_id).await.unwrap()
    // });

    // this loop ends when we unsubscribe
    while let Some(Message::UserNonFundingLedgerUpdates(user_non_funding_ledger_update)) =
        receiver.recv().await
    {
        info!("Received user non funding ledger update data: {user_non_funding_ledger_update:?}");
    }
}
