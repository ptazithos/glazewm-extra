use anyhow::Result;
use fastwebsockets::handshake;
use fastwebsockets::WebSocket;
use http_body_util::Empty;
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::tokio::TokioIo;

use crate::spawn_executor::SpawnExecutor;

use tokio::net::TcpStream;

pub async fn connect() -> Result<WebSocket<TokioIo<Upgraded>>> {
    let stream = TcpStream::connect("localhost:6123").await?;

    let req = Request::builder()
        .method("GET")
        .uri("http://localhost:6123/")
        .header("Host", "localhost:6123")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header(
            "Sec-WebSocket-Key",
            fastwebsockets::handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, stream).await?;
    Ok(ws)
}
