use clap::{command, Parser};
use ipc::{command::get_windows, subscribe::subscribe};
use native::set_window_alpha;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 128)]
    alpha: u8,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = Args::parse();

    let _ = subscribe("focus_changed", |payload| {
        let alpha = Args::parse().alpha;
        let focused_handle = payload.data.focused_container.handle;

        tokio::spawn(async move {
            let windows = get_windows().await.unwrap();
            windows
                .data
                .iter()
                .for_each(|window| set_window_alpha(window.handle, alpha));

            set_window_alpha(focused_handle, 255);
        });

        false
    })
    .await;
}
