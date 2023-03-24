#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod timer;
use std::sync::{Arc, Mutex};
use tauri::api::notification::Notification;
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn send_notification(identifier: &str, message: &str) {
    let count = 1 + COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let body = format!("[{}] {}", count, message);
    Notification::new(identifier).body(body).show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("open", "Open"))
                    .add_item(CustomMenuItem::new("hide", "Hide"))
                    .add_item(CustomMenuItem::new("exit", "Exit")),
            ),
        )
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "exit" => {
                    let mut shortcut_manager = app.global_shortcut_manager();
                    shortcut_manager.unregister_all().unwrap();
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::Ready => {
                let timer = Arc::new(Mutex::new(timer::CountUpTimer::new()));
                timer.lock().unwrap().start();
                let mut shortcut_manager = app.global_shortcut_manager();

                if !shortcut_manager.is_registered("PageUp").unwrap() {
                    let identifier = app.config().tauri.bundle.identifier.clone();
                    shortcut_manager
                        .register("PageUp", move || {
                            timer.lock().unwrap().stop();
                            let time = timer.lock().unwrap().get_history_last();
                            send_notification(identifier.as_str(), time.as_str());
                            timer.lock().unwrap().start();
                        })
                        .unwrap();
                }
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
