use ipc::{command::get_windows, subscribe::subscribe};
use native::set_window_alpha;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = subscribe("focus_changed", |payload| {
        let focused_handle = payload.data.focused_container.handle;

        tokio::spawn(async move {
            let windows = get_windows().await.unwrap();
            windows
                .data
                .iter()
                .for_each(|window| set_window_alpha(window.handle, 122));

            set_window_alpha(focused_handle, 255);
        });

        false
    })
    .await;
}
