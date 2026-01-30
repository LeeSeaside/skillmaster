use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String,
    pub source_type: SourceType,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub status: SkillStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Local,
    Git,
    Zip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillStatus {
    Active,
    Inactive,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTool {
    pub tool_id: String,
    pub tool_name: String,
    pub is_installed: bool,
    pub sync_status: SyncStatus,
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Synced,
    Pending,
    Error,
    NotSynced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDetail {
    #[serde(flatten)]
    pub skill: Skill,
    pub tools: Vec<SkillTool>,
    pub manifest: Option<SkillManifest>,
    pub directory_tree: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub skill_md_content: Option<String>,
}
