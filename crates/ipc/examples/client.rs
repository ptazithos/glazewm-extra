use anyhow::{Ok, Result};
use ipc::subscribe::subscribe;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    subscribe("focus_changed", |payload| {
        println!("Received message: {:?}", payload);
        false
    })
    .await?;
    Ok(())
}
