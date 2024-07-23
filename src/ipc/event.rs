use std::sync::{Arc, Mutex};

use super::websocket::Stream;
use anyhow::Result;

pub struct EventRegistry {
    callbacks: Arc<Mutex<Vec<fn(payload: &str)>>>,
}

impl EventRegistry {
    pub fn new() -> EventRegistry {
        EventRegistry {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register(self, callback: fn(payload: &str)) -> Self {
        {
            let mut callbacks = self.callbacks.lock().unwrap();
            callbacks.push(callback);
        }
        return self;
    }

    pub async fn listen(self) -> Result<()> {
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
