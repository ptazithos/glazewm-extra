use super::websocket::Stream;
use crate::service::EventRegistry;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct IPCEventRegistry {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&str) + Send>>>>,
}

impl IPCEventRegistry {
    pub fn new() -> IPCEventRegistry {
        IPCEventRegistry {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventRegistry for IPCEventRegistry {
    fn register<F>(&mut self, callback: F)
    where
        F: Fn(&str) + 'static + Send,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    async fn listen(&self) -> Result<()> {
        let mut stream = Stream::new().await?;
        let callbacks_mutex = self.callbacks.clone();
        let res = tokio::spawn(async move {
            stream.write("subscribe -e all").await?;
            loop {
                let res = stream.read().await?;
                let callbacks = callbacks_mutex.lock().unwrap();
                for callback in callbacks.iter() {
                    callback(&res);
                }
            }
        });

        res.await?
    }
}
