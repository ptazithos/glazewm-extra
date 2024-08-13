use super::websocket::Stream;
use anyhow::{anyhow, Result};
use serde::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CommandExecutor {
    stream: Arc<Mutex<Option<Stream>>>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {
            stream: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn execute(&self, command: &str) -> Result<String> {
        let mut stream_guard = self.stream.lock().await;
        if stream_guard.is_none() {
            *stream_guard = Some(Stream::new().await?);
        }

        let stream = stream_guard.as_mut().unwrap();

        stream.write(command).await?;

        let res = match tokio::time::timeout(std::time::Duration::from_secs(5), stream.read()).await
        {
            Ok(result) => match result {
                Ok(response) => Ok(response),
                Err(e) => Err(anyhow!("Stream read error: {}", e)),
            },
            Err(_) => Err(anyhow!("Timeout reading from stream")),
        }?;

        Ok(res)
    }
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
    static EXECUTOR: tokio::sync::OnceCell<CommandExecutor> = tokio::sync::OnceCell::const_new();
    let executor = EXECUTOR
        .get_or_init(|| async { CommandExecutor::new() })
        .await;

    let response = executor.execute("query windows").await?;
    let parsed: Payload = serde_json::from_str(&response)?;

    Ok(parsed)
}
