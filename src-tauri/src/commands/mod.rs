pub mod skills;
pub mod sync;
pub mod tools;

pub use skills::*;
pub use sync::*;
pub use tools::*;

use crate::services::detector;

#[tauri::command]
pub fn check_directory_exists(path: String) -> bool {
    detector::check_directory_exists(&path)
}
