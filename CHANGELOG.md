# 更新日志

所有重要的项目更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 计划中
- 应用启动时自动检查更新
- 更新签名验证
- 增量更新支持
- 更新进度显示
- 技能搜索和过滤功能
- 技能分类和标签管理
- 批量操作支持
- 导出和备份功能

## [0.1.0] - 2024-01-30

### 新增
- ✨ 初始版本发布
- ✨ 技能集中管理功能
- ✨ 支持从 ZIP 文件导入技能
- ✨ 支持从 Git 仓库导入技能
- ✨ 支持子目录导入
- ✨ 自动检测已安装的 AI 工具
- ✨ 一键同步技能到所有工具
- ✨ 单个工具技能同步
- ✨ 技能详情查看
- ✨ 技能删除功能
- ✨ 自定义工具添加
- ✨ 手动检查更新功能
- ✨ 自动下载和安装更新
- ✨ 单实例应用限制
- ✨ 现代化 UI 界面
- ✨ 多平台支持（Windows、macOS、Linux）

### 支持的 AI 工具
- Cursor
- Claude Code
- Codex
- OpenCode
- Antigravity
- Amp
- Kilo Code
- Roo Code
- Goose
- Gemini CLI
- GitHub Copilot
- Clawdbot
- Droid
- Windsurf

### 技术栈
- Tauri 2.0
- Vue 3
- TypeScript
- Tailwind CSS
- Pinia
- Rust

### 文档
- 📚 完整的使用文档
- 📚 多平台构建指南
- 📚 版本管理文档
- 📚 发布检查清单
- 📚 GitHub Actions 自动化

---

## 版本说明

### 版本号格式
`主版本号.次版本号.修订号`

- **主版本号**: 不兼容的 API 修改
- **次版本号**: 向下兼容的功能性新增
- **修订号**: 向下兼容的问题修正

### 更新类型

- `新增` - 新功能
- `修改` - 现有功能的更改
- `弃用` - 即将移除的功能
- `移除` - 已移除的功能
- `修复` - Bug 修复
- `安全` - 安全性修复

---

[未发布]: https://github.com/LeeSeaside/skillmaster/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/LeeSeaside/skillmaster/releases/tag/v0.1.0
