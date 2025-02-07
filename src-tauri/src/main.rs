// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod goannot;
use tauri::generate_handler;
use goannot::process_file; 

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![process_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



