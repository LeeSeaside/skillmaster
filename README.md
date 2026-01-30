# SkillMaster

<div align="center">

![SkillMaster Logo](app-icon.svg)

**AI 技能中央管理工具**

一个强大的桌面应用，用于集中管理和同步 AI 编程工具的技能包

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/LeeSeaside/skillmaster/releases)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/LeeSeaside/skillmaster/releases)

[English](README.md) | [简体中文](README_CN.md)

</div>

## ✨ 特性

- 🎯 **集中管理** - 统一管理所有 AI 工具的技能包
- 🔄 **一键同步** - 快速同步技能到多个 AI 工具
- 📦 **多种导入方式** - 支持 ZIP 文件和 Git 仓库导入
- 🛠️ **自动检测** - 自动检测已安装的 AI 编程工具
- 🔍 **技能详情** - 查看技能的完整信息和文件结构
- 🗑️ **安全删除** - 从中央仓库和所有工具中移除技能
- 🔔 **自动更新** - 内置更新检查和自动安装功能
- 🎨 **现代界面** - 基于 Vue 3 + Tailwind CSS 的美观界面
- 🚀 **高性能** - 使用 Tauri 构建，体积小、速度快
- 🌍 **跨平台** - 支持 Windows、macOS 和 Linux

## 🎬 快速开始

### 下载安装

前往 [Releases](https://github.com/LeeSeaside/skillmaster/releases) 页面下载适合你系统的安装包：

- **Windows**: `SkillMaster_x.x.x_x64_en-US.msi`
- **macOS**: `SkillMaster_x.x.x_x64.dmg` (Intel) 或 `SkillMaster_x.x.x_aarch64.dmg` (Apple Silicon)
- **Linux**: `skillmaster_x.x.x_amd64.AppImage` 或 `skillmaster_x.x.x_amd64.deb`

### 支持的 AI 工具

SkillMaster 支持以下 AI 编程工具：

- ✅ Cursor
- ✅ Claude Code
- ✅ Codex
- ✅ OpenCode
- ✅ Antigravity
- ✅ Amp
- ✅ Kilo Code
- ✅ Roo Code
- ✅ Goose
- ✅ Gemini CLI
- ✅ GitHub Copilot
- ✅ Clawdbot
- ✅ Droid
- ✅ Windsurf
- 🔧 支持添加自定义工具

## 📖 使用指南

### 1. 导入技能

1. 点击"技能管理"页面的"导入技能"按钮
2. 选择导入方式：
   - **ZIP 文件**: 选择本地 ZIP 文件
   - **Git 仓库**: 输入 Git 仓库 URL
3. 等待导入完成

### 2. 同步技能

1. 进入"AI 工具"页面
2. 点击"一键同步所有技能"或为单个工具同步
3. 等待同步完成

### 3. 管理技能

- **查看详情**: 点击技能卡片的"查看详情"按钮
- **删除技能**: 点击"删除"按钮（会从所有工具中移除）

### 4. 添加自定义工具

1. 进入"AI 工具"页面
2. 点击"添加工具"按钮
3. 填写工具信息：
   - 工具 ID（唯一标识符）
   - 显示名称
   - 检测目录（相对于用户主目录）
   - 技能目录（相对于用户主目录）

## 🛠️ 开发

### 环境要求

- Node.js 20+
- Rust 1.70+
- npm 或 yarn

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri:dev
```

### 构建

```bash
npm run tauri:build
```

### 项目结构

```
skillmaster/
├── src/                    # Vue 前端代码
│   ├── components/         # UI 组件
│   ├── views/             # 页面视图
│   ├── stores/            # Pinia 状态管理
│   └── types/             # TypeScript 类型定义
├── src-tauri/             # Tauri 后端代码
│   ├── src/               # Rust 源代码
│   │   ├── commands/      # Tauri 命令
│   │   ├── models/        # 数据模型
│   │   ├── services/      # 业务逻辑
│   │   └── utils/         # 工具函数
│   └── Cargo.toml         # Rust 依赖配置
├── docs/                  # 文档
└── .github/workflows/     # GitHub Actions
```

## 📚 文档

- [安装指南](INSTALL.md)
- [更新指南](UPDATE_GUIDE.md)
- [多平台构建](docs/MULTI_PLATFORM_BUILD.md)
- [版本管理](docs/VERSION_MANAGEMENT.md)
- [发布检查清单](RELEASE_CHECKLIST.md)
- [技能导入说明](docs/IMPORT_SKILLS.md)
- [支持的工具列表](docs/SUPPORTED_TOOLS.md)

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📝 更新日志

查看 [CHANGELOG.md](CHANGELOG.md) 了解版本历史。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Tailwind CSS](https://tailwindcss.com/) - CSS 框架
- [shadcn/ui](https://ui.shadcn.com/) - UI 组件库

## 📧 联系方式

- GitHub Issues: [https://github.com/LeeSeaside/skillmaster/issues](https://github.com/LeeSeaside/skillmaster/issues)
- 项目主页: [https://github.com/LeeSeaside/skillmaster](https://github.com/LeeSeaside/skillmaster)

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐️ Star！**

Made with ❤️ by SkillMaster Team

</div>
