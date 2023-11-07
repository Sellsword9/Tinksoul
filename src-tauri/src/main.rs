// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// imports
use markdown;
use std::{io::Write, fs::File};
use tauri::{command, generate_handler, Builder};
const PREVIEW_PATH: &str = "../temp/preview.md";


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn markdownize(md: &str) -> String {
    let mut preview_file = File::create(PREVIEW_PATH).expect("Unable to create file 1");
    File::write(&mut preview_file, md.as_bytes()).expect("Unable to write file to disk 2");
    markdown::to_html(md)
}
#[command]
fn close() -> () {
    std::process::exit(1);
}
fn main() {
    
    Builder::default()
        .invoke_handler(generate_handler![markdownize, close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}