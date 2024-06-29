use super::stream::Stream;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    #[serde(rename(deserialize = "subscriptionId"))]
    subscription_id: String,
    success: bool,
    #[serde(rename(deserialize = "messageType"))]
    message_type: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename(deserialize = "focusedContainer"))]
    pub focused_container: Container,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub handle: isize,
    id: String,
}

pub async fn subscribe(event: &str, callback: fn(&Payload) -> bool) -> anyhow::Result<()> {
    let mut stream = Stream::new().await?;
    stream
        .write(format!("subscribe -e {}", event).as_str())
        .await?;

    loop {
        let payload_str = stream.read().await?;

        match serde_json::from_str(&payload_str) {
            Ok(payload) => {
                callback(&payload);
            }
            Err(error) => {
                println!("{:?}", error)
            }
        }
    }
}
