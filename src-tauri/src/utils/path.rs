use std::path::PathBuf;

pub fn get_repo_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    Ok(home.join(".skillmaster"))
}

pub fn get_skills_path() -> Result<PathBuf, String> {
    Ok(get_repo_path()?.join("skills"))
}

pub fn get_config_path() -> Result<PathBuf, String> {
    Ok(get_repo_path()?.join("config.json"))
}

pub fn ensure_repo_dirs() -> Result<(), String> {
    let repo_path = get_repo_path()?;
    let skills_path = get_skills_path()?;
    
    std::fs::create_dir_all(&repo_path)
        .map_err(|e| format!("Failed to create repo directory: {}", e))?;
    std::fs::create_dir_all(&skills_path)
        .map_err(|e| format!("Failed to create skills directory: {}", e))?;
    
    Ok(())
}
