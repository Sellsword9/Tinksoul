#![allow(dead_code)]
/**
 * Accepts a string with the path to a brain and returns the path to the folder
*/
pub fn get_brain_path(b: &str) -> String {
  // Panic if not a tksbrain file
  if !b.ends_with(".tksbrain") {
    panic!("{}", NOT_A_BRAIN_ERROR);
  }
  // Panic if not a path
  if !b.contains("/") {
    panic!("{}", NOT_A_PATH_ERROR);
  }
  // Get the path to the brain
  let mut path: String = b.to_string();
  // Remove the filename
  path.truncate(path.rfind('/').unwrap_or(usize::MAX));
  return path;
}
pub const NOT_A_BRAIN_ERROR: &str = "Not a brain file";
pub const NOT_A_PATH_ERROR: &str = "Not a valid path. (Use /, instead of \\)";