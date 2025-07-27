// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_system;
mod mpv;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // From file_system.rs
            file_system::get_videos,
            // From mpv.rs
            mpv::play,
            mpv::pause,
            mpv::resume,
            mpv::seek,
            mpv::set_volume,
            mpv::toggle_fullscreen,
            mpv::stop 
        ])
        .setup(|app| {
            mpv::setup_mpv(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}