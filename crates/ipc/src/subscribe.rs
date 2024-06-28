use crate::stream::Stream;
use anyhow::{Ok, Result};


pub async fn subscribe(event: &str, callback: fn(&str) -> bool) -> Result<()> {
    let mut stream = Stream::new().await?;
    stream
        .write(format!("subscribe -e {}", event).as_str())
        .await?;

    loop {
        let response = stream.read().await?;
        if callback(&response) {
            break;
        }
    }

    Ok(())
}
