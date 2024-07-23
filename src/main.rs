mod ipc;

use ipc::EventRegistry;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _ = EventRegistry::new()
        .register(|payload| {
            println!("Payload: {}", payload);
        })
        .listen()
        .await;

    println!("Hello!");
    loop {}
}
