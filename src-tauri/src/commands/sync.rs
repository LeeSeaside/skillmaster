use crate::models::SyncResult;
use crate::services::sync;

#[tauri::command]
pub fn sync_skill(
    skill_id: String,
    tool_id: String,
    use_symlink: bool,
) -> Result<SyncResult, String> {
    sync::sync_skill(&skill_id, &tool_id, use_symlink, None)
}

#[tauri::command]
pub fn sync_skill_with_path(
    skill_id: String,
    tool_id: String,
    skills_dir: String,
    use_symlink: bool,
) -> Result<SyncResult, String> {
    sync::sync_skill(&skill_id, &tool_id, use_symlink, Some(skills_dir))
}

#[tauri::command]
pub fn unsync_skill(skill_id: String, tool_id: String) -> Result<(), String> {
    sync::unsync_skill(&skill_id, &tool_id, None)
}

#[tauri::command]
pub fn unsync_skill_with_path(
    skill_id: String,
    tool_id: String,
    skills_dir: String,
) -> Result<(), String> {
    sync::unsync_skill(&skill_id, &tool_id, Some(skills_dir))
}
