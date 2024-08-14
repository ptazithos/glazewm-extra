use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::{spawn, sync::mpsc};
use tray_item::{IconSource, TrayItem};

use crate::{ipc::get_windows, service::EventRegistry};

pub struct TrayEventRegistry {
    tray: TrayItem,
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&str, &Vec<isize>) + Send>>>>,
}

impl TrayEventRegistry {
    pub fn new() -> Self {
        TrayEventRegistry {
            tray: TrayItem::new("glazewm-extra", IconSource::Resource("tray-default")).unwrap(),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventRegistry for TrayEventRegistry {
    fn register<F>(&mut self, callback: F)
    where
        F: Fn(&str, &Vec<isize>) + Send + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    async fn listen(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<()>(12);

        self.tray
            .add_menu_item("Quit", move || {
                tx.blocking_send(()).unwrap();
            })
            .unwrap();

        let callbacks_mutex = self.callbacks.clone();
        let res = spawn(async move {
            loop {
                rx.recv().await.unwrap();
                let payload = get_windows().await?;
                let hwnds = payload
                    .data
                    .windows
                    .iter()
                    .map(|c| c.handle)
                    .collect::<Vec<_>>();

                let callbacks = callbacks_mutex.lock().unwrap();
                for callback in callbacks.iter() {
                    callback("quit", &hwnds);
                }
            }
        });

        res.await?
    }
}
