#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

pub mod monitor;
pub mod pb;
use monitor::battery_info;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let system_tray = SystemTray::new();

    tauri::Builder::default()
        .system_tray(system_tray)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            native_windows(&main_window, Some(10.));
            Ok(())
        })
        .on_system_tray_event(system_tray_handler)
        .invoke_handler(tauri::generate_handler![greet, battery_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn system_tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position, size: _, ..
        } => {
            let window = app.get_window("main").unwrap();
            window.set_position(position).unwrap();
            #[cfg(target_os = "windows")]
            window.center().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { tray_id: _, id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn native_windows(window: &tauri::Window, vibrancy_radius: Option<f64>) {
    // use window_shadows::set_shadow;
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    // set_shadow(window, true).unwrap();

    apply_vibrancy(
        window,
        NSVisualEffectMaterial::Sidebar,
        None,
        vibrancy_radius,
    )
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
}
