use tauri::{AppHandle, Manager};

pub fn command_execute(app: &AppHandle, argv: Vec<String>) {
    if argv.len() <= 1 {
        return;
    }

    let command = argv[1].as_str();

    match command {
        "workspaces" => {
            let _ = app.emit_to("overview", "trigger-overview", Option::<()>::None);
        }
        _ => {}
    };
}
