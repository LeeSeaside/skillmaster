export interface Skill {
  id: string
  name: string
  description: string
  source: string
  sourceType: 'local' | 'git' | 'zip'
  version: string
  createdAt: string
  updatedAt: string
  tags: string[]
  status: 'active' | 'inactive' | 'error'
}

export interface SkillTool {
  toolId: string
  toolName: string
  isInstalled: boolean
  syncStatus: 'synced' | 'pending' | 'error' | 'not_synced'
  lastSyncedAt?: string
}

export interface SkillDetail extends Skill {
  tools: SkillTool[]
  manifest?: SkillManifest
}

export interface SkillManifest {
  name: string
  version: string
  description: string
  author?: string
  tags?: string[]
}

export interface Tool {
  id: string
  name: string
  displayName: string
  skillsDir: string
  detectDir: string
  isInstalled: boolean
  installPath?: string
  isCustom?: boolean
}

export interface SyncResult {
  success: boolean
  message: string
  method?: 'symlink' | 'copy'
}

export interface DetectedTool {
  id: string
  name: string
  path: string
  skills_dir: string
  is_installed: boolean
}
