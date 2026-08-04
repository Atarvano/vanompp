pub mod commands;
pub mod conf;
pub mod projects;
pub mod services;
pub mod utils;

use services::ServiceState;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ServiceState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::projects::scan_projects,
            commands::projects::scan_projects_with_port,
            commands::projects::create_project,
            commands::projects::create_project_with_port,
            commands::projects::open_project_folder,
            commands::services::start_service,
            commands::services::stop_service,
            commands::services::get_status,
            commands::services::check_ports,
            commands::services::start_all_services,
            commands::services::stop_all_services,
            commands::db::create_database,
            commands::db::create_db,
            commands::logs::read_log
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let state = window.state::<ServiceState>();
                services::kill_all(&state);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
