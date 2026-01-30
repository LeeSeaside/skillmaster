use crate::models::DetectedTool;
use crate::services::detector;

#[tauri::command]
pub fn detect_installed_tools() -> Vec<DetectedTool> {
    detector::detect_installed_tools()
}
