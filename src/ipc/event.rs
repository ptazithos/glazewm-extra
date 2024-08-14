use super::{command::get_windows, websocket::Stream};
use crate::service::EventRegistry;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct IPCEventRegistry {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&str, &Vec<isize>) + Send>>>>,
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
        F: Fn(&str, &Vec<isize>) + 'static + Send,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    async fn listen(&mut self) -> Result<()> {
        let mut stream = Stream::new().await?;
        let callbacks_mutex = self.callbacks.clone();
        let res = tokio::spawn(async move {
            stream.write("sub -e all").await?;

            loop {
                let res = stream.read().await?;

                let payload = get_windows().await?;
                let hwnds = payload
                    .data
                    .windows
                    .iter()
                    .map(|c| c.handle)
                    .collect::<Vec<_>>();

                let callbacks = callbacks_mutex.lock().unwrap();
                for callback in callbacks.iter() {
                    callback(&res, &hwnds);
                }
            }
        });

        res.await?
    }
}
