use super::websocket::Stream;
use anyhow::Result;
use serde::*;

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
    #[serde(rename(deserialize = "messageType"))]
    message_type: String,
    error: Option<String>,
    success: bool,

    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    pub windows: Vec<Window>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub handle: isize,
    id: String,
}

pub async fn get_windows() -> Result<Payload> {
    let res = command("query windows").await?;
    let payload: Payload = serde_json::from_str(&res).unwrap();
    Ok(payload)
}
