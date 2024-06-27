use anyhow::{Ok, Result};
use fastwebsockets::FragmentCollector;
use fastwebsockets::Frame;
use fastwebsockets::Payload;
use ipc::connect;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let ws = connect().await?;
    let mut collector = FragmentCollector::new(ws);

    collector
        .write_frame(Frame::text(Payload::Borrowed(&"workspaces".as_bytes())))
        .await?;

    let frame = collector.read_frame().await?;

    let payload = String::from_utf8(frame.payload.to_vec()).expect("Invalid UTF-8 data");

    println!("{:?}", payload);
    Ok(())
}
