#![allow(dead_code)]
use std::fs;
pub const MAIN_BRAIN_PATH: &str = "../brains/brain0.tksbrain";
pub const NOT_A_BRAIN_ERROR: &str = "Not a brain file";
pub const NOT_A_PATH_ERROR: &str = "Not a valid path. (Use /, instead of \\)";

/**
 * Accepts a string with the path to a brain and returns the path to the folder
*/
pub fn get_brain_path(b: &str) -> String {
    {
        // Panics brackets
        // Panic if not a tksbrain file
        if !b.ends_with(".tksbrain") {
            panic!("{}", NOT_A_BRAIN_ERROR);
        }
        // Panic if not a path
        if !b.contains("/") {
            panic!("{}", NOT_A_PATH_ERROR);
        }
    }

    // Get the path to the brain
    let mut path: String = b.to_string();
    // Remove the filename
    path.truncate(path.rfind('/').unwrap_or(usize::MAX));
    return path;
}

/**
 * Accepts a string with the name of a Brain,
 * read the main brain file,
 * returns the relative path to the brain file itself.
 * Panics if brain not found. Panics if brain0.tksbrain is corrupted.
 * Either panics or returns.
 * @param brain: The name of the brain file. For example: "CustomName101.tksbrain". Do NOT include / nor ./ nor .tksbrain
 * @param contain_brains_folder: Whether the return value should be like ./brains/filename.tksbrain instead of ./filename.tksbrain
 * @returns The relative path to the brain file. For example: "./brains/User-Made-Folder/CustomName101.tksbrain"
 */
fn get_path_of_brain_file(brain: &str, contain_brains_folder: &bool) -> String {
    // Declare path, which will be ./ or ./brains/. Will be used at return statement
    let mut path: String = String::from("./");
    if *contain_brains_folder {
        path.push_str("brains/");
    }
    // read the main brain for the path
    let brains_text: String = fs::read_to_string(MAIN_BRAIN_PATH).expect("Unable to read file");

    // Get the path to the brain & return it
    let index: Option<usize> = brains_text.find(brain);

    // Slice the string from index until a & is found
    if index.is_none() {
        panic!("Brain not found");
    } else {
        let rest_of_file = brains_text.get(index.unwrap()..);
        if !rest_of_file.unwrap_or("").contains("&&") {
            panic!("Brain name found, but brain0.tksbrain is corrupted. Try manually adding && after the path of brain name");
        } else {
            let real_rest = rest_of_file.unwrap();
            let path_end_index: Option<usize> = rest_of_file.unwrap().find("&&");
            let true_end_index = path_end_index.unwrap();
            return format!("{}{}", path, real_rest.get(0..true_end_index).unwrap());
            // TODO: Check if this is an early return or a panic-or-return return
        }
    }
}
fn get_clean_path_of_brain_file(brain: &str) -> String {
    return get_path_of_brain_file(brain, &false);
}
