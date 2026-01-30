use crate::models::{Skill, SkillDetail, SkillManifest, SkillStatus, SourceType};
use crate::utils::{get_skills_path, ensure_repo_dirs, copy_dir_all, remove_dir_all_safe, extract_zip};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

pub fn get_skills() -> Result<Vec<Skill>, String> {
    ensure_repo_dirs()?;
    let skills_path = get_skills_path()?;
    
    let mut skills = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&skills_path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Ok(skill) = load_skill_from_dir(&entry.path()) {
                    skills.push(skill);
                }
            }
        }
    }
    
    Ok(skills)
}

pub fn get_skill_detail(skill_id: &str) -> Result<SkillDetail, String> {
    eprintln!("Getting skill detail for: {}", skill_id);
    let skills_path = get_skills_path()?;
    let skill_path = skills_path.join(skill_id);
    
    if !skill_path.exists() {
        return Err(format!("Skill not found: {}", skill_id));
    }
    
    eprintln!("Loading skill from dir...");
    let skill = load_skill_from_dir(&skill_path)?;
    
    eprintln!("Loading manifest...");
    let manifest = load_manifest(&skill_path).ok();
    
    eprintln!("Generating directory tree...");
    // Generate directory tree with error handling
    let directory_tree = match generate_directory_tree(&skill_path) {
        Ok(tree) => {
            eprintln!("Directory tree generated successfully");
            Some(tree)
        }
        Err(e) => {
            eprintln!("Failed to generate directory tree: {}", e);
            None
        }
    };
    
    eprintln!("Skill detail loaded successfully");
    Ok(SkillDetail {
        skill,
        tools: Vec::new(), // TODO: Load tool sync status
        manifest,
        directory_tree,
    })
}

pub fn import_skill(source: &str, source_type: SourceType) -> Result<String, String> {
    import_skill_with_subdir(source, source_type, None)
}

pub fn import_skill_with_subdir(
    source: &str,
    source_type: SourceType,
    subdir: Option<String>,
) -> Result<String, String> {
    ensure_repo_dirs()?;
    let skills_path = get_skills_path()?;
    
    // Debug logging
    eprintln!("Importing skill:");
    eprintln!("  Source: {}", source);
    eprintln!("  Type: {:?}", source_type);
    eprintln!("  Subdir: {:?}", subdir);
    
    // First, copy to a temporary location to read the manifest
    let temp_id = Uuid::new_v4().to_string();
    let temp_path = skills_path.join(&temp_id);
    
    match source_type {
        SourceType::Local => {
            let source_path = Path::new(source);
            if !source_path.exists() {
                return Err(format!("Source path does not exist: {}", source));
            }
            
            let actual_source = if let Some(ref sub) = subdir {
                source_path.join(sub)
            } else {
                source_path.to_path_buf()
            };
            
            if !actual_source.exists() {
                return Err(format!("Subdirectory '{}' does not exist", subdir.unwrap_or_default()));
            }
            
            copy_dir_all(&actual_source, &temp_path)
                .map_err(|e| format!("Failed to copy skill: {}", e))?;
        }
        SourceType::Zip => {
            let source_path = Path::new(source);
            if !source_path.exists() {
                return Err(format!("ZIP file does not exist: {}", source));
            }
            
            // Extract to temporary directory first
            let extract_temp_dir = std::env::temp_dir().join(format!("skillmaster-zip-{}", Uuid::new_v4()));
            eprintln!("  Extracting ZIP to temp dir: {:?}", extract_temp_dir);
            
            extract_zip(&source_path, &extract_temp_dir)
                .map_err(|e| format!("Failed to extract ZIP: {}", e))?;
            
            // If subdir specified, copy only that subdirectory
            let source_to_copy = if let Some(ref sub) = subdir {
                let subdir_path = extract_temp_dir.join(sub);
                if !subdir_path.exists() {
                    remove_dir_all_safe(&extract_temp_dir).ok();
                    return Err(format!("Subdirectory '{}' not found in ZIP file", sub));
                }
                subdir_path
            } else {
                // Check if ZIP has a single root directory
                let entries: Vec<_> = fs::read_dir(&extract_temp_dir)
                    .map_err(|e| format!("Failed to read temp directory: {}", e))?
                    .filter_map(|e| e.ok())
                    .collect();
                
                if entries.len() == 1 && entries[0].path().is_dir() {
                    // Single root directory, use it as source
                    entries[0].path()
                } else {
                    // Multiple files/dirs at root, use extract_temp_dir as source
                    extract_temp_dir.clone()
                }
            };
            
            copy_dir_all(&source_to_copy, &temp_path)
                .map_err(|e| format!("Failed to copy skill: {}", e))?;
            
            // Clean up temp directory
            remove_dir_all_safe(&extract_temp_dir).ok();
        }
        SourceType::Git => {
            // Clone to temporary directory first
            let temp_dir = std::env::temp_dir().join(format!("skillmaster-{}", Uuid::new_v4()));
            eprintln!("  Cloning to temp dir: {:?}", temp_dir);
            
            clone_git_repo(source, &temp_dir)?;
            
            // If subdir specified, copy only that subdirectory
            let source_to_copy = if let Some(ref sub) = subdir {
                let subdir_path = temp_dir.join(sub);
                if !subdir_path.exists() {
                    remove_dir_all_safe(&temp_dir).ok();
                    return Err(format!("Subdirectory '{}' not found in repository", sub));
                }
                subdir_path
            } else {
                temp_dir.clone()
            };
            
            copy_dir_all(&source_to_copy, &temp_path)
                .map_err(|e| format!("Failed to copy skill: {}", e))?;
            
            // Clean up temp directory
            remove_dir_all_safe(&temp_dir).ok();
        }
    }
    
    // Try to get a friendly name from subdir or source
    let default_name = if let Some(ref sub) = subdir {
        // Use subdirectory name
        sub.split('/').last()
            .or_else(|| sub.split('\\').last())
            .unwrap_or("skill")
            .to_string()
    } else {
        // Try to extract repo name from Git URL or path
        let source_name = source
            .trim_end_matches(".git")
            .split('/')
            .last()
            .or_else(|| source.split('\\').last())
            .unwrap_or("skill");
        source_name.to_string()
    };
    
    // Load manifest to get the actual skill name
    let manifest = load_manifest(&temp_path).unwrap_or_else(|_| SkillManifest {
        name: default_name.clone(),
        version: "1.0.0".to_string(),
        description: "Imported skill".to_string(),
        author: None,
        tags: None,
        skill_md_content: None,
    });
    
    // Create a safe directory name from the skill name
    let safe_name = sanitize_directory_name(&manifest.name);
    
    // Generate a short unique suffix (first 8 chars of UUID)
    let unique_suffix = &temp_id[..8];
    let skill_id = format!("{}-{}", safe_name, unique_suffix);
    
    // Rename from temp path to final path
    let target_path = skills_path.join(&skill_id);
    
    // If target already exists (shouldn't happen with UUID suffix), use full UUID
    let final_path = if target_path.exists() {
        let fallback_id = Uuid::new_v4().to_string();
        skills_path.join(&fallback_id)
    } else {
        target_path
    };
    
    fs::rename(&temp_path, &final_path)
        .map_err(|e| format!("Failed to rename skill directory: {}", e))?;
    
    let final_skill_id = final_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&temp_id)
        .to_string();
    
    let skill = Skill {
        id: final_skill_id.clone(),
        name: manifest.name.clone(),
        description: manifest.description.clone(),
        source: if let Some(ref sub) = subdir {
            format!("{}#{}", source, sub)
        } else {
            source.to_string()
        },
        source_type,
        version: manifest.version.clone(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        tags: manifest.tags.unwrap_or_default(),
        status: SkillStatus::Active,
    };
    
    save_skill_metadata(&final_path, &skill)?;
    
    Ok(final_skill_id)
}

fn sanitize_directory_name(name: &str) -> String {
    // Replace invalid characters with hyphens
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .trim_matches('_')
        .to_lowercase()
}

pub fn delete_skill(skill_id: &str) -> Result<(), String> {
    let skills_path = get_skills_path()?;
    let skill_path = skills_path.join(skill_id);
    
    if !skill_path.exists() {
        return Err(format!("Skill not found: {}", skill_id));
    }
    
    // First, unsync from all tools
    let tools = crate::services::detector::detect_installed_tools();
    for tool in tools {
        // Try to unsync, but don't fail if it doesn't exist
        // Use None for custom_skills_dir since we're using detected tools
        crate::services::sync::unsync_skill(skill_id, &tool.id, None).ok();
    }
    
    // Then delete from central repository
    remove_dir_all_safe(&skill_path)
        .map_err(|e| format!("Failed to delete skill: {}", e))?;
    
    Ok(())
}

fn load_skill_from_dir(path: &Path) -> Result<Skill, String> {
    let metadata_path = path.join(".skillmaster.json");
    
    if metadata_path.exists() {
        let content = fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse metadata: {}", e))
    } else {
        // Create default skill from directory name
        let skill_id = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Ok(Skill {
            id: skill_id.clone(),
            name: skill_id,
            description: "No description".to_string(),
            source: "unknown".to_string(),
            source_type: SourceType::Local,
            version: "1.0.0".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            tags: Vec::new(),
            status: SkillStatus::Active,
        })
    }
}

fn load_manifest(path: &Path) -> Result<SkillManifest, String> {
    let manifest_path = path.join("SKILL.md");
    
    eprintln!("Loading manifest from: {:?}", manifest_path);
    
    if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read SKILL.md: {}", e))?;
        
        eprintln!("SKILL.md content length: {}", content.len());
        
        // Parse front matter if exists
        let (name, version, description, author, tags) = parse_skill_md_frontmatter(&content);
        
        eprintln!("Parsed manifest - name: {:?}, version: {:?}", name, version);
        
        Ok(SkillManifest {
            name: name.unwrap_or_else(|| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            }),
            version: version.unwrap_or_else(|| "1.0.0".to_string()),
            description: description.unwrap_or_else(|| "Skill".to_string()),
            author,
            tags,
            skill_md_content: Some(content),
        })
    } else {
        eprintln!("SKILL.md not found");
        Err("Manifest not found".to_string())
    }
}

fn parse_skill_md_frontmatter(content: &str) -> (Option<String>, Option<String>, Option<String>, Option<String>, Option<Vec<String>>) {
    let mut name = None;
    let mut version = None;
    let mut description = None;
    let mut author = None;
    let mut tags = None;
    
    // Check if content starts with front matter (---)
    if content.starts_with("---") {
        if let Some(end_idx) = content[3..].find("---") {
            let frontmatter = &content[3..end_idx + 3];
            
            for line in frontmatter.lines() {
                let line = line.trim();
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_lowercase();
                    let value = value.trim().trim_matches('"').trim_matches('\'');
                    
                    match key.as_str() {
                        "name" => name = Some(value.to_string()),
                        "version" => version = Some(value.to_string()),
                        "description" => description = Some(value.to_string()),
                        "author" => author = Some(value.to_string()),
                        "tags" => {
                            // Parse tags as comma-separated or array
                            if value.starts_with('[') && value.ends_with(']') {
                                let tags_str = value.trim_matches('[').trim_matches(']');
                                tags = Some(
                                    tags_str
                                        .split(',')
                                        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                                        .filter(|s| !s.is_empty())
                                        .collect()
                                );
                            } else {
                                tags = Some(
                                    value
                                        .split(',')
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty())
                                        .collect()
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    // If no description in frontmatter, try to extract from content
    if description.is_none() {
        // Find first paragraph after frontmatter or first heading
        let content_start = if content.starts_with("---") {
            content[3..].find("---").map(|i| i + 6).unwrap_or(0)
        } else {
            0
        };
        
        let remaining = &content[content_start..];
        for line in remaining.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                description = Some(line.to_string());
                break;
            }
        }
    }
    
    (name, version, description, author, tags)
}

fn generate_directory_tree(path: &Path) -> Result<String, String> {
    let mut tree = String::new();
    
    // Try to get skill name from metadata or manifest
    let skill_name = if let Ok(skill) = load_skill_from_dir(path) {
        skill.name
    } else {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string()
    };
    
    tree.push_str(&format!("{}\n", skill_name));
    
    if path.is_dir() {
        generate_tree_recursive(path, &mut tree, "", 0)?;
    }
    
    Ok(tree)
}

fn generate_tree_recursive(
    path: &Path,
    output: &mut String,
    prefix: &str,
    depth: usize,
) -> Result<(), String> {
    // Limit depth to prevent infinite recursion
    if depth > 5 {
        return Ok(());
    }
    
    let mut entries: Vec<_> = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            // Filter out hidden files and common ignore patterns
            if let Some(name) = e.file_name().to_str() {
                !name.starts_with('.') 
                    && name != "node_modules" 
                    && name != "target"
                    && name != "__pycache__"
            } else {
                false
            }
        })
        .collect();
    
    // Sort: directories first, then files
    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });
    
    let total = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        output.push_str(&format!("{}{}{}\n", prefix, connector, file_name_str));
        
        if entry.path().is_dir() {
            let extension = if is_last { "    " } else { "│   " };
            let new_prefix = format!("{}{}", prefix, extension);
            generate_tree_recursive(&entry.path(), output, &new_prefix, depth + 1)?;
        }
    }
    
    Ok(())
}

fn save_skill_metadata(path: &Path, skill: &Skill) -> Result<(), String> {
    let metadata_path = path.join(".skillmaster.json");
    let content = serde_json::to_string_pretty(skill)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
    
    fs::write(&metadata_path, content)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;
    
    Ok(())
}

fn clone_git_repo(url: &str, target: &Path) -> Result<(), String> {
    use git2::{FetchOptions, RemoteCallbacks};
    
    eprintln!("Cloning repository: {}", url);
    eprintln!("Target directory: {:?}", target);
    
    // Create callbacks for progress reporting
    let mut callbacks = RemoteCallbacks::new();
    callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            eprintln!(
                "Resolving deltas {}/{}",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            eprintln!(
                "Received {}/{} objects ({} bytes)",
                stats.received_objects(),
                stats.total_objects(),
                stats.received_bytes()
            );
        }
        true
    });
    
    // Configure fetch options with timeout
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    // Create builder with fetch options
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    
    // Clone with timeout handling
    builder
        .clone(url, target)
        .map_err(|e| {
            let error_msg = format!("Failed to clone repository: {}", e);
            eprintln!("{}", error_msg);
            
            // Provide more helpful error messages
            if error_msg.contains("timeout") || error_msg.contains("超时") {
                format!(
                    "克隆仓库超时。可能的原因：\n\
                    1. 网络连接不稳定\n\
                    2. 仓库太大\n\
                    3. 防火墙或代理设置\n\
                    \n建议：\n\
                    - 检查网络连接\n\
                    - 尝试使用本地已克隆的仓库\n\
                    - 配置 Git 代理（如果需要）\n\
                    \n原始错误: {}", e
                )
            } else if error_msg.contains("SSL") || error_msg.contains("certificate") {
                format!(
                    "SSL 证书验证失败。\n\
                    建议：配置 Git 忽略 SSL 验证（不推荐）或安装正确的证书\n\
                    \n原始错误: {}", e
                )
            } else if error_msg.contains("authentication") || error_msg.contains("401") || error_msg.contains("403") {
                format!(
                    "身份验证失败。\n\
                    建议：\n\
                    - 检查仓库 URL 是否正确\n\
                    - 确保仓库是公开的，或配置 Git 凭据\n\
                    \n原始错误: {}", e
                )
            } else {
                error_msg
            }
        })?;
    
    eprintln!("Clone completed successfully");
    Ok(())
}
