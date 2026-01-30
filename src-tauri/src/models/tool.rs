use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub skills_dir: String,
    pub detect_dir: String,
    pub is_installed: bool,
    pub install_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedTool {
    pub id: String,
    pub name: String,
    pub path: String,
    pub skills_dir: String,
    pub is_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub method: Option<SyncMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncMethod {
    Symlink,
    Copy,
}
