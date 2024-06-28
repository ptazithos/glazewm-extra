use anyhow::{Ok, Result};
use ipc::command::get_windows;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let payload = get_windows().await?;
    let handles: Vec<isize> = payload.data.iter().map(|window| window.handle).collect();
    print!("{:?}", handles);
    Ok(())
}
