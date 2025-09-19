use std::time::Duration;

use pusher_rs::{PusherClient, PusherConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PusherConfig::from_env()?;
    let mut client = PusherClient::new(config)?;

    client.connect().await?;

    client.subscribe("my-channel").await?;

    client
        .bind("my-event", |event| {
            println!("Received event: {:#?}", event);
        })
        .await?;
    println!("Binded");
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
