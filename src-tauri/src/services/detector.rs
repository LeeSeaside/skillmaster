use crate::models::DetectedTool;
use std::path::PathBuf;

pub fn detect_installed_tools() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    
    if let Some(home) = dirs::home_dir() {
        // Define all supported tools with their detection and skills directories
        let tool_configs = vec![
            ("cursor", "Cursor", ".cursor", ".cursor/skills"),
            ("claude_code", "Claude Code", ".claude", ".claude/skills"),
            ("codex", "Codex", ".codex", ".codex/skills"),
            ("opencode", "OpenCode", ".config/opencode", ".config/opencode/skill"),
            ("antigravity", "Antigravity", ".gemini/antigravity", ".gemini/antigravity/skills"),
            ("amp", "Amp", ".config/agents", ".config/agents/skills"),
            ("kilo_code", "Kilo Code", ".kilocode", ".kilocode/skills"),
            ("roo_code", "Roo Code", ".roo", ".roo/skills"),
            ("goose", "Goose", ".config/goose", ".config/goose/skills"),
            ("gemini_cli", "Gemini CLI", ".gemini", ".gemini/skills"),
            ("github_copilot", "GitHub Copilot", ".copilot", ".copilot/skills"),
            ("clawdbot", "Clawdbot", ".clawdbot", ".clawdbot/skills"),
            ("droid", "Droid", ".factory", ".factory/skills"),
            ("windsurf", "Windsurf", ".codeium/windsurf", ".codeium/windsurf/skills"),
        ];
        
        for (id, name, detect_dir, skills_dir) in tool_configs {
            let detect_path = build_path(&home, detect_dir);
            if detect_path.exists() {
                let skills_path = build_path(&home, skills_dir);
                tools.push(DetectedTool {
                    id: id.to_string(),
                    name: name.to_string(),
                    path: detect_path.to_string_lossy().to_string(),
                    skills_dir: skills_path.to_string_lossy().to_string(),
                    is_installed: true,
                });
            }
        }
    }
    
    tools
}

pub fn check_directory_exists(path: &str) -> bool {
    if let Some(home) = dirs::home_dir() {
        let full_path = build_path(&home, path);
        full_path.exists()
    } else {
        false
    }
}

pub fn get_tool_skills_dir(tool_id: &str) -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    
    let skills_dir = match tool_id {
        "cursor" => ".cursor/skills",
        "claude_code" => ".claude/skills",
        "codex" => ".codex/skills",
        "opencode" => ".config/opencode/skill",
        "antigravity" => ".gemini/antigravity/skills",
        "amp" => ".config/agents/skills",
        "kilo_code" => ".kilocode/skills",
        "roo_code" => ".roo/skills",
        "goose" => ".config/goose/skills",
        "gemini_cli" => ".gemini/skills",
        "github_copilot" => ".copilot/skills",
        "clawdbot" => ".clawdbot/skills",
        "droid" => ".factory/skills",
        "windsurf" => ".codeium/windsurf/skills",
        _ => return Err(format!("Unknown tool: {}", tool_id)),
    };
    
    let path = build_path(&home, skills_dir);
    
    // Create the directory if it doesn't exist
    if !path.exists() {
        eprintln!("Creating skills directory for {}: {:?}", tool_id, path);
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create skills directory for {}: {}", tool_id, e))?;
    }
    
    Ok(path)
}

fn build_path(base: &PathBuf, relative: &str) -> PathBuf {
    let parts: Vec<&str> = relative.split('/').collect();
    let mut path = base.clone();
    for part in parts {
        path = path.join(part);
    }
    path
}
