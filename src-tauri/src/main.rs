// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::utils::config;

mod confighandler;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      confighandler::load_config,
      confighandler::save_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
