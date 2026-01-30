// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod utils;

use commands::*;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 当尝试打开第二个实例时，聚焦到已存在的窗口
            let windows = app.webview_windows();
            if let Some(window) = windows.values().next() {
                let _ = window.set_focus();
                let _ = window.unminimize();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            get_skills,
            get_skill_detail,
            import_skill,
            import_skill_with_subdir,
            delete_skill,
            sync_skill,
            sync_skill_with_path,
            unsync_skill,
            unsync_skill_with_path,
            detect_installed_tools,
            check_directory_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
