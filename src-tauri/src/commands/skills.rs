use crate::models::{Skill, SkillDetail, SourceType};
use crate::services::repo;

#[tauri::command]
pub fn get_skills() -> Result<Vec<Skill>, String> {
    repo::get_skills()
}

#[tauri::command]
pub fn get_skill_detail(skill_id: String) -> Result<SkillDetail, String> {
    repo::get_skill_detail(&skill_id)
}

#[tauri::command]
pub fn import_skill(source: String, source_type: SourceType) -> Result<String, String> {
    repo::import_skill(&source, source_type)
}

#[tauri::command]
pub fn import_skill_with_subdir(
    source: String,
    source_type: SourceType,
    subdir: Option<String>,
) -> Result<String, String> {
    repo::import_skill_with_subdir(&source, source_type, subdir)
}

#[tauri::command]
pub fn delete_skill(skill_id: String) -> Result<(), String> {
    repo::delete_skill(&skill_id)
}
