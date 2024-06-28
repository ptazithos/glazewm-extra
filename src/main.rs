use ipc::subscribe::subscribe;
use native::set_window_alpha;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = subscribe("focus_changed", |payload| {
        set_window_alpha(payload.data.focused_container.handle, 220);
        false
    })
    .await;
}
