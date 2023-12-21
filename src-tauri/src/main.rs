// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// TODO: remove us
#![allow(dead_code)]
#![allow(unused_imports)]
// imports
mod brainparser;
use brainparser::MAIN_BRAIN_PATH;
use markdown;
use serde::de::value::Error;
use std::{fs::File, io::Write}; // fmt::format
use tauri::{command, generate_handler, Builder};
const PREVIEW_PATH: &str = "../temp/preview.md";


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn setup() -> bool {
    File::create(MAIN_BRAIN_PATH).expect("Unable to create file");
    true
}
#[command]
fn markdownize(md: &str) -> String {
    save_file(md, PREVIEW_PATH);
    markdown::to_html(md)
}

fn close() -> () {
    std::process::exit(1);
}

//TODO: Move all helper functions outside main --> Main will be used for tauri commands only

#[command]
fn execute(command: &str, content: &str, path: &str) -> () {
    let bfolder: &str = brainparser::MAIN_BRAIN_FOLDER;
    let complete_path: String = format!("{}/{}", bfolder, path);
    print!("Executing command: {}", command);
    print!("Content: {}", content);
    print!("Path: {}", complete_path);
    let mut clean_command: String = command.replace(":", "");
    clean_command = clean_command.replace(" ", "");
    clean_command = clean_command.to_lowercase();
    let c: &str = &clean_command;
    match c {
        "q" => close(),
        "w" => save_file(content, complete_path.as_str()),
        _ => {
            panic!("Command not found");
        }
    }
}

fn save_file(content: &str, path: &str) -> () {
    let mut file: File = File::create(path).expect("Unable to create file");
    File::write(&mut file, content.as_bytes()).expect("Unable to write file to disk");
}

pub fn main() {
    Builder::default()
        .invoke_handler(generate_handler![markdownize, execute, setup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    // use crate::brainparser::get_brain_path;
    // #[test]
    // fn test_get_brain_path() {
    //     let x = get_brain_path("C:/Users/username/Documents/brain.tksbrain");
    //     assert_eq!(x, "C:/Users/username/Documents");
    // }
    // #[test]
    // fn test_get_brain_path2() {
    //     let x = get_brain_path("./username/Documents/brain12123.tksbrain");
    //     assert_eq!(x, "./username/Documents");
    // }
    // #[test]
    // #[should_panic(expected = "Not a brain file")]
    // fn test_get_brain_path3() {
    //     get_brain_path("./username/Documents/");
    // }
    // // Let copilot create more tests
    // #[test]
    // fn test_get_brain_path4() {
    //     let x = get_brain_path("C:/Users/username/Documents/brain.tksbrain");
    //     assert_eq!(x, "C:/Users/username/Documents");
    // }
    // #[test]
    // fn test_get_brain_path5() {
    //     let x = get_brain_path("./username/Documents/brain12123.tksbrain");
    //     assert_eq!(x, "./username/Documents");
    // }
    // #[test]
    // #[should_panic(expected = "Not a brain file")]
    // fn test_get_brain_path6() {
    //     get_brain_path("./username/Documents/");
    // }
    // #[test]
    // #[should_panic(expected = "Not a valid path. (Use /, instead of \\)")]
    // fn test_get_brain_path7() {
    //     let _x = get_brain_path("brain.tksbrain");
    // }
    // #[test]
    // fn test_get_brain_path_long() {
    //     let x = get_brain_path(
    //         "./username/username/username/hola/doc/username/braintsstsdfsdfsdf.tksbrain",
    //     );
    //     assert_eq!(x, "./username/username/username/hola/doc/username");
    // }
}
