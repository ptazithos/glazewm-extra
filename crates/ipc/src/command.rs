use anyhow::Ok;
use serde::*;

use crate::Stream;

pub async fn command(command: &str) -> anyhow::Result<String> {
    let mut stream = Stream::new().await?;
    stream.write(command).await?;
    let res = stream.read().await?;

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn get_windows() -> anyhow::Result<Payload> {
    let res = command("windows").await?;
    let payload: Payload = serde_json::from_str(&res).unwrap();
    Ok(payload)
}
