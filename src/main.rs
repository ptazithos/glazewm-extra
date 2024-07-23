mod ipc;

use ipc::EventRegistry;
use tokio::select;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let res = EventRegistry::new()
        .register(|payload| {
            println!("Payload: {}", payload);
        })
        .listen();

    select! {
        _ = res => {},
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down...");
        }
    }
}
