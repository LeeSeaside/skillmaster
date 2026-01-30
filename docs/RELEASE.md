# 发布指南

本文档包含版本发布的完整流程、构建命令和检查清单。

## 📋 目录

- [快速发布流程](#快速发布流程)
- [版本号管理](#版本号管理)
- [构建命令](#构建命令)
- [发布检查清单](#发布检查清单)
- [平台支持](#平台支持)

---

## 快速发布流程

### 1. 修改版本号

同时修改两个文件：

**src-tauri/tauri.conf.json**
```json
{
  "version": "0.2.0"
}
```

**src-tauri/Cargo.toml**
```toml
[package]
version = "0.2.0"
```

### 2. 使用 GitHub Actions 发布（推荐）

```bash
# 1. 提交更改
git add .
git commit -m "Release v0.2.0"

# 2. 创建标签
git tag v0.2.0

# 3. 推送标签（自动触发构建）
git push origin v0.2.0
```

GitHub Actions 会自动：
- 为所有平台构建应用
- 创建 GitHub Release
- 上传安装包
- 生成 `latest.json`

### 3. 本地构建（可选）

```bash
# 当前平台
npm run tauri:build

# macOS 特定架构
npm run tauri:build -- --target aarch64-apple-darwin
npm run tauri:build -- --target universal-apple-darwin
```

---

## 版本号管理

遵循语义化版本：`主版本号.次版本号.修订号`

- **修订号** (0.1.x)：Bug 修复，向下兼容
- **次版本号** (0.x.0)：新功能，向下兼容
- **主版本号** (x.0.0)：重大更新，可能不兼容

示例：
- 0.1.0 → 0.1.1（修复 bug）
- 0.1.1 → 0.2.0（新增功能）
- 0.2.0 → 1.0.0（重大更新）

---

## 构建命令

### 支持的平台

| 平台 | 架构 | 格式 | 文件名示例 |
|------|------|------|-----------|
| Windows | x86_64 | MSI/NSIS | `SkillMaster_0.1.0_x64_en-US.msi` |
| macOS | x86_64 | DMG | `SkillMaster_0.1.0_x64.dmg` |
| macOS | ARM64 | DMG | `SkillMaster_0.1.0_aarch64.dmg` |
| Linux | x86_64 | AppImage/DEB | `skillmaster_0.1.0_amd64.AppImage` |

### 本地构建

```bash
# Windows
npm run tauri:build

# macOS (Intel)
npm run tauri:build

# macOS (Apple Silicon)
npm run tauri:build -- --target aarch64-apple-darwin

# macOS (通用版本)
npm run tauri:build -- --target universal-apple-darwin

# Linux
npm run tauri:build
```

### 构建产物位置

```
src-tauri/target/release/bundle/
├── msi/              # Windows MSI
├── nsis/             # Windows NSIS
├── dmg/              # macOS DMG
├── macos/            # macOS .app
├── appimage/         # Linux AppImage
└── deb/              # Linux DEB
```

---

## 发布检查清单

### 发布前准备

#### 1. 版本号更新
- [ ] 更新 `src-tauri/tauri.conf.json` 中的 `version`
- [ ] 更新 `src-tauri/Cargo.toml` 中的 `version`
- [ ] 确保两个文件版本号一致

#### 2. 代码质量检查
- [ ] 运行测试：`npm test`
- [ ] 检查 TypeScript：`npm run build`
- [ ] 检查 Rust：`cargo check`
- [ ] 代码格式化：`npm run format`

#### 3. 功能测试
- [ ] 技能导入功能正常
- [ ] 技能同步功能正常
- [ ] 技能删除功能正常
- [ ] 工具检测功能正常
- [ ] 更新检查功能正常

#### 4. 文档更新
- [ ] 更新 CHANGELOG.md
- [ ] 更新 README.md（如有必要）
- [ ] 检查文档链接有效

### 构建和测试

#### 5. 本地构建测试
- [ ] Windows 构建成功
- [ ] macOS 构建成功（如有条件）
- [ ] Linux 构建成功（如有条件）
- [ ] 安装包大小合理（< 30MB）

#### 6. 安装测试
- [ ] Windows MSI 安装正常
- [ ] macOS DMG 安装正常
- [ ] Linux AppImage 运行正常

#### 7. 功能回归测试
- [ ] 新安装的应用功能正常
- [ ] 数据存储正常
- [ ] 窗口状态记忆正常

### 发布流程

#### 8. Git 操作
```bash
git add .
git commit -m "Release v0.x.x"
git tag v0.x.x
git push origin main
git push origin v0.x.x
```

#### 9. GitHub Actions
- [ ] 等待所有平台构建完成
- [ ] 检查构建日志无错误
- [ ] 验证所有 Artifacts 已生成

#### 10. GitHub Release
- [ ] Release 自动创建成功
- [ ] 所有平台安装包已上传
- [ ] `latest.json` 文件已上传
- [ ] Release 说明清晰完整

#### 11. 验证下载链接
- [ ] Windows 下载链接有效
- [ ] macOS x86_64 下载链接有效
- [ ] macOS ARM64 下载链接有效
- [ ] Linux 下载链接有效

### 更新测试

#### 12. 更新功能测试
- [ ] 从旧版本更新到新版本
- [ ] 更新检查功能正常
- [ ] 更新下载功能正常
- [ ] 更新安装功能正常
- [ ] 用户数据保留完整

### 发布后

#### 13. 通知和监控
- [ ] 在 GitHub Release 发布公告
- [ ] 监控 GitHub Issues
- [ ] 收集用户反馈
- [ ] 记录已知问题

---

## 平台支持

### Windows
- Windows 10 或更高版本
- Visual Studio Build Tools

### macOS
- macOS 10.13 或更高版本
- Xcode Command Line Tools

### Linux
- Ubuntu 20.04+ / Debian 11+ / Fedora 35+
- 必需的系统库：
  ```bash
  sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev \
    libappindicator3-dev librsvg2-dev patchelf libssl-dev
  ```

---

## latest.json 模板

```json
{
  "version": "0.2.0",
  "notes": "更新内容说明",
  "pub_date": "2024-01-30T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "url": "https://github.com/你的用户名/skillmaster/releases/download/v0.2.0/SkillMaster_0.2.0_x64_en-US.msi"
    },
    "darwin-x86_64": {
      "url": "https://github.com/你的用户名/skillmaster/releases/download/v0.2.0/SkillMaster_0.2.0_x64.dmg"
    },
    "darwin-aarch64": {
      "url": "https://github.com/你的用户名/skillmaster/releases/download/v0.2.0/SkillMaster_0.2.0_aarch64.dmg"
    },
    "linux-x86_64": {
      "url": "https://github.com/你的用户名/skillmaster/releases/download/v0.2.0/skillmaster_0.2.0_amd64.AppImage"
    }
  }
}
```

---

## 故障排除

### 构建失败
```bash
cargo clean
npm clean-install
npm run tauri:build
```

### macOS 签名问题
```bash
export TAURI_SKIP_SIGNING=true
npm run tauri:build
```

### Linux 缺少依赖
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev \
  libappindicator3-dev librsvg2-dev patchelf libssl-dev
```

---

## 性能优化

在 `Cargo.toml` 中添加：

```toml
[profile.release]
lto = true              # 链接时优化
codegen-units = 1       # 更好的优化
opt-level = "z"         # 优化大小
strip = true            # 移除调试符号
panic = "abort"         # 减小二进制大小
```

---

## 回滚计划

如果发现严重问题：

1. **立即行动**
   - 在 Release 中标记为 "Pre-release"
   - 添加警告说明

2. **修复问题**
   - 创建热修复分支
   - 修复并快速测试

3. **发布修复版本**
   - 增加修订号（如 0.2.0 → 0.2.1）
   - 快速发布并通知用户

---

## 相关文档

- [开发文档](DEVELOPMENT.md) - 开发指南
- [GitHub Actions](../.github/workflows/release.yml) - CI/CD 配置

---

最后更新: 2026-01-30
