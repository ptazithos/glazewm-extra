use anyhow::{Ok, Result};
use serde::*;

use super::websocket::Stream;

pub async fn command(command: &str) -> Result<String> {
    let mut stream = Stream::new().await?;
    stream.write(command).await?;
    let res = stream.read().await?;
    stream.close().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Payload {
    #[serde(rename(deserialize = "clientMessage"))]
    client_message: String,
    success: bool,
    #[serde(rename(deserialize = "messageType"))]
    message_type: String,
    pub data: Vec<Container>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub handle: isize,
    id: String,
}

pub async fn get_windows() -> Result<Payload> {
    let res = command("windows").await?;
    let payload: Payload = serde_json::from_str(&res).unwrap();
    Ok(payload)
}
