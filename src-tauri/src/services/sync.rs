use crate::models::{SyncResult, SyncMethod};
use crate::services::detector::get_tool_skills_dir;
use crate::utils::{get_skills_path, copy_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn sync_skill(
    skill_id: &str,
    tool_id: &str,
    use_symlink: bool,
    custom_skills_dir: Option<String>,
) -> Result<SyncResult, String> {
    let skills_path = get_skills_path()?;
    let skill_path = skills_path.join(skill_id);
    
    if !skill_path.exists() {
        return Err(format!("Skill not found: {}", skill_id));
    }
    
    let tool_skills_dir = if let Some(custom_dir) = custom_skills_dir {
        // Use custom directory for custom tools
        let home = dirs::home_dir().ok_or("Failed to get home directory")?;
        build_path_from_string(&home, &custom_dir)
    } else {
        // Use built-in tool detection
        get_tool_skills_dir(tool_id)?
    };
    
    let target_path = tool_skills_dir.join(skill_id);
    
    // Check if target already exists
    if target_path.exists() {
        return Ok(SyncResult {
            success: false,
            message: "Target already exists (non-destructive mode)".to_string(),
            method: None,
        });
    }
    
    // Ensure all parent directories exist (including tool base directory)
    eprintln!("Creating tool skills directory: {:?}", tool_skills_dir);
    std::fs::create_dir_all(&tool_skills_dir)
        .map_err(|e| format!("Failed to create tool skills directory '{}': {}. Please ensure the AI tool is properly installed.", tool_skills_dir.display(), e))?;
    
    // Verify the directory was created
    if !tool_skills_dir.exists() {
        return Err(format!("Tool skills directory '{}' does not exist after creation. Please check permissions.", tool_skills_dir.display()));
    }
    
    eprintln!("Tool skills directory verified: {:?}", tool_skills_dir);
    
    if use_symlink {
        // Try to create junction (Windows symlink for directories)
        match create_junction(&skill_path, &target_path) {
            Ok(_) => Ok(SyncResult {
                success: true,
                message: "Synced using junction".to_string(),
                method: Some(SyncMethod::Symlink),
            }),
            Err(e) => {
                // Fallback to copy
                eprintln!("Junction failed: {}, falling back to copy", e);
                sync_by_copy(&skill_path, &target_path)
            }
        }
    } else {
        sync_by_copy(&skill_path, &target_path)
    }
}

fn build_path_from_string(base: &PathBuf, relative: &str) -> PathBuf {
    let parts: Vec<&str> = relative.split(&['/', '\\'][..]).collect();
    let mut path = base.clone();
    for part in parts {
        if !part.is_empty() {
            path = path.join(part);
        }
    }
    path
}

fn create_junction(source: &Path, target: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        let source_str = source.to_string_lossy();
        let target_str = target.to_string_lossy();
        
        let output = Command::new("cmd")
            .args(&["/C", "mklink", "/J", &target_str, &source_str])
            .output()
            .map_err(|e| format!("Failed to execute mklink: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("mklink failed: {}", stderr));
        }
        
        Ok(())
    }
    
    #[cfg(not(windows))]
    {
        Err("Junction is only supported on Windows".to_string())
    }
}

fn sync_by_copy(source: &Path, target: &Path) -> Result<SyncResult, String> {
    copy_dir_all(source, target)
        .map_err(|e| format!("Failed to copy directory: {}", e))?;
    
    Ok(SyncResult {
        success: true,
        message: "Synced using copy".to_string(),
        method: Some(SyncMethod::Copy),
    })
}

pub fn unsync_skill(skill_id: &str, tool_id: &str, custom_skills_dir: Option<String>) -> Result<(), String> {
    let tool_skills_dir = if let Some(custom_dir) = custom_skills_dir {
        // Use custom directory for custom tools
        let home = dirs::home_dir().ok_or("Failed to get home directory")?;
        build_path_from_string(&home, &custom_dir)
    } else {
        // Use built-in tool detection
        get_tool_skills_dir(tool_id)?
    };
    
    let target_path = tool_skills_dir.join(skill_id);
    
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)
            .map_err(|e| format!("Failed to remove synced skill: {}", e))?;
    }
    
    Ok(())
}
