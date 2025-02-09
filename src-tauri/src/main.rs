// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod goannot;
use tauri::generate_handler;
use tauri::{WindowBuilder, command};
use std::env;

use goannot::process_file; 

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![process_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



#[tauri::command]
fn open_stats_window() {
    let is_dev = env::var("TAURI_ENV").unwrap_or_default() == "development";
    
    let url = if is_dev {
        "http://localhost:3000/new-window" // Development URL
    } else {
        "tauri://localhost/new-window" // Production URL (local file access)
    };
    /*  Create the new window
    let handle = tauri::AppHandle::clone();
    std::thread::spawn(move || {
        let window = WindowBuilder::new(&handle, "new-window")
            .title("New Window")
            .url(url)
            .build()
            .unwrap();

        // Additional window-specific logic if needed
    });*/
  }



