use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn generate_menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new().add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    SystemTray::new().with_menu(tray_menu)
}

pub fn menu_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                let _ = app.emit_all("clean_quit", ());
            }
            _ => {}
        },
        _ => {}
    }
}
