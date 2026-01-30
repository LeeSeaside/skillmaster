# 支持的 AI 编码工具

SkillMaster 支持以下 AI 编码工具的技能同步：

## 工具列表

| 工具 ID | 显示名称 | 技能目录 (相对于 `~`) | 检测目录 (相对于 `~`) |
| --- | --- | --- | --- |
| `cursor` | Cursor | `.cursor/skills` | `.cursor` |
| `claude_code` | Claude Code | `.claude/skills` | `.claude` |
| `codex` | Codex | `.codex/skills` | `.codex` |
| `opencode` | OpenCode | `.config/opencode/skill` | `.config/opencode` |
| `antigravity` | Antigravity | `.gemini/antigravity/skills` | `.gemini/antigravity` |
| `amp` | Amp | `.config/agents/skills` | `.config/agents` |
| `kilo_code` | Kilo Code | `.kilocode/skills` | `.kilocode` |
| `roo_code` | Roo Code | `.roo/skills` | `.roo` |
| `goose` | Goose | `.config/goose/skills` | `.config/goose` |
| `gemini_cli` | Gemini CLI | `.gemini/skills` | `.gemini` |
| `github_copilot` | GitHub Copilot | `.copilot/skills` | `.copilot` |
| `clawdbot` | Clawdbot | `.clawdbot/skills` | `.clawdbot` |
| `droid` | Droid | `.factory/skills` | `.factory` |
| `windsurf` | Windsurf | `.codeium/windsurf/skills` | `.codeium/windsurf` |

## 工作原理

### 检测

SkillMaster 会自动检测已安装的 AI 工具：

1. 扫描用户主目录 (`~`) 下的检测目录
2. 如果检测目录存在，则认为该工具已安装
3. 在"AI工具"页面显示已检测到的工具

### 同步

当你将技能同步到某个 AI 工具时：

1. SkillMaster 会将技能从中央仓库复制到该工具的技能目录
2. 默认使用符号链接（symlink）以节省空间
3. 如果符号链接失败，会回退到完整复制

### 添加新工具

如果你想添加对新 AI 工具的支持，请编辑：

- `src-tauri/src/services/detector.rs` - 添加检测和路径配置
- 本文档 - 更新工具列表

## 注意事项

- 所有路径都相对于用户主目录 (`~`)
- Windows 系统中，`~` 通常是 `C:\Users\<用户名>`
- macOS/Linux 系统中，`~` 通常是 `/home/<用户名>` 或 `/Users/<用户名>`
- 某些工具可能需要手动创建技能目录
