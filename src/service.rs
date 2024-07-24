use std::future::Future;

use tokio::select;

pub trait EventRegistry {
    fn register(&mut self, callback: fn(payload: &str));
    fn listen(&self) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub struct EffectService<T: EventRegistry> {
    pub ipc: T,
}
impl<T: EventRegistry> EffectService<T> {
    pub fn new(mut ipc: T) -> Self {
        ipc.register(|payload| {
            println!("Payload: {}", payload);
        });
        EffectService { ipc }
    }

    pub async fn serve(&self) {
        let ipc_registry = &self.ipc;
        let ipc_fut = ipc_registry.listen();

        select! {
            _ = ipc_fut => {},
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down...");
            }
        }
    }
}
