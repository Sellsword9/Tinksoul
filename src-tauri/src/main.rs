// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// TODO: remove me
#![allow(dead_code)]
// imports
mod brainparser;
use brainparser::{get_brain_path, get_number_of_brain, MAIN_BRAIN_PATH};
use markdown;
use std::{fs::File, io::Write}; // fmt::format
use tauri::{command, generate_handler, Builder};
const PREVIEW_PATH: &str = "../temp/preview.md";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn setup() -> bool {
    File::create(MAIN_BRAIN_PATH).expect("Unable to create file");
    return true;
}
#[command]
fn markdownize(md: &str) -> String {
    save_file(md, PREVIEW_PATH);
    markdown::to_html(md)
}
#[command]
fn close() -> () {
    std::process::exit(1);
}
/**
 * @param filename: The name of the file to save. For example: "readme" (would save as readme.md)
 * @param brainpath: The path to the brain file. For example: "C:/Users/username/Documents/brain.tksbrain"
 * @param content: The content to save, in plain text. For example: "# Hello world"
 */
#[command]
fn save_on_brain(filename: &str, brainpath: &str, content: &str) -> bool {
    print!("Trace: {}, {}, {}", filename, brainpath, content);
    let real_path: String = get_brain_path(brainpath);
    let real_name: String = format!("{}/{}.md", real_path, filename);
    save_file(content, &real_name);
    return true;
}

fn save_file(content: &str, p: &str) -> () {
    let mut file: File = File::create(p).expect("Unable to create file");
    File::write(&mut file, content.as_bytes()).expect("Unable to write file to disk");
}

fn write_path(newpath: &str, filename: &str) -> () {
    let mut file: File = File::create(MAIN_BRAIN_PATH).expect("Unable to create file");
    let x = get_number_of_brain(&newpath);
    let content: String = format!("{} {} {}", x, newpath, filename);
    file.write(content.as_bytes()).unwrap();
}

pub fn main() {
    Builder::default()
        .invoke_handler(generate_handler![markdownize, close, setup, save_on_brain])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::brainparser::get_brain_path;

    #[test]
    fn test_get_brain_path() {
        let x = get_brain_path("C:/Users/username/Documents/brain.tksbrain");
        assert_eq!(x, "C:/Users/username/Documents");
    }
    #[test]
    fn test_get_brain_path2() {
        let x = get_brain_path("./username/Documents/brain12123.tksbrain");
        assert_eq!(x, "./username/Documents");
    }
    #[test]
    #[should_panic(expected = "Not a brain file")]
    fn test_get_brain_path3() {
        get_brain_path("./username/Documents/");
    }
    // Let copilot create more tests
    #[test]
    fn test_get_brain_path4() {
        let x = get_brain_path("C:/Users/username/Documents/brain.tksbrain");
        assert_eq!(x, "C:/Users/username/Documents");
    }
    #[test]
    fn test_get_brain_path5() {
        let x = get_brain_path("./username/Documents/brain12123.tksbrain");
        assert_eq!(x, "./username/Documents");
    }
    #[test]
    #[should_panic(expected = "Not a brain file")]
    fn test_get_brain_path6() {
        get_brain_path("./username/Documents/");
    }
    #[test]
    #[should_panic(expected = "Not a valid path. (Use /, instead of \\)")]
    fn test_get_brain_path7() {
        let _x = get_brain_path("brain.tksbrain");
    }
    #[test]
    fn test_get_brain_path_long() {
        let x = get_brain_path(
            "./username/username/username/hola/doc/username/braintsstsdfsdfsdf.tksbrain",
        );
        assert_eq!(x, "./username/username/username/hola/doc/username");
    }
}
