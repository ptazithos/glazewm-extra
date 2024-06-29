mod ipc;
mod native;

use clap::{command, Parser};
use ipc::{command::get_windows, subscribe::subscribe};
use native::{set_window_alpha, set_window_titlebar};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 128)]
    alpha: u8,
    #[arg(long, action)]
    hide_titlebar: bool,
    #[arg(long, action)]
    enable_transparency: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = Args::parse();

    let _ = subscribe("focus_changed", |payload| {
        let Args {
            alpha,
            hide_titlebar,
            enable_transparency,
        } = Args::parse();

        let focused_handle = payload.data.focused_container.handle;

        tokio::spawn(async move {
            let windows = get_windows().await.unwrap();
            windows.data.iter().for_each(|window| {
                if enable_transparency {
                    set_window_alpha(window.handle, alpha);
                }
                if hide_titlebar {
                    set_window_titlebar(window.handle, false);
                }
            });
            if enable_transparency {
                set_window_alpha(focused_handle, 255);
            }
        });

        false
    })
    .await;
}
