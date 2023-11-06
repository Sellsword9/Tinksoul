// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// imports
use markdown;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn test_md(md: &str) -> String {
    markdown::to_html(md)
}
#[tauri::command]
fn close(md: &str) -> () {
    std::process::exit(1);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_md])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
