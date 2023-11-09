// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// imports
use markdown;
use std::{io::Write, fs::File};
use tauri::{command, generate_handler, Builder};
const PREVIEW_PATH: &str = "../temp/preview.md";


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn setup() -> bool
{
    File::create("../brains/brain0.tksbrain").expect("Unable to create file");
    return true;
}
#[command]
fn markdownize(md: &str) -> String {
    save_file(md, PREVIEW_PATH);
    //fixed: Markdown crashes on "* "
    markdown::to_html(md)
}
#[command]
fn close() -> () {
    std::process::exit(1);
}
#[command]
fn save_file(f: &str, p: &str) -> () {
    let mut file: File = File::create(p).expect("Unable to create file");
    File::write(&mut file, f.as_bytes()).expect("Unable to write file to disk");
}
fn main() {
    
    Builder::default()
        .invoke_handler(generate_handler![markdownize, close, save_file, setup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}