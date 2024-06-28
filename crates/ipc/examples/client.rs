use anyhow::{Ok, Result};
use ipc::subscribe::subscribe;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    subscribe("focus_changed", |message| {
        println!("Received message: {}", message);
        message == "quit"
    })
    .await?;
    Ok(())
}
