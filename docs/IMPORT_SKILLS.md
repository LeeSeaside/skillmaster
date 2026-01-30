# 技能导入指南

## 📦 支持的导入方式

SkillMaster 支持多种方式导入技能：

### 1. 导入 ZIP 文件 ⭐ 推荐

```
选择 ZIP 文件，点击"浏览"按钮选择本地的 .zip 文件
子目录: (可选，如果 ZIP 包含多个技能)
```

这是最简单快捷的导入方式，适合：
- 从其他地方下载的技能包
- 备份的技能文件
- 离线导入场景

**ZIP 文件结构**：
- 如果 ZIP 只有一个根目录，会自动使用该目录
- 如果 ZIP 有多个文件/目录，可以指定子目录

### 2. 从 Git 仓库导入特定子目录

```
Git URL: https://github.com/mrgoonie/claudekit-skills
子目录: chrome-devtools
```

这会只导入仓库中的 `chrome-devtools` 子目录作为技能。

**等同于命令**：
```bash
npx skills add https://github.com/mrgoonie/claudekit-skills --skill chrome-devtools
```

### 3. 导入整个 Git 仓库

```
Git URL: https://github.com/user/skills-repo.git
子目录: (留空)
```

这会克隆整个仓库作为一个技能。

## 🎯 使用示例

### 示例 1：导入 ZIP 文件（推荐）

1. 打开"技能管理"页面
2. 点击"导入技能"
3. 选择"ZIP 文件"
4. 点击"浏览"按钮，选择你的 .zip 文件
   - 或者直接输入完整路径：`C:\Downloads\my-skill.zip`
5. 如果 ZIP 包含多个技能，在"子目录"中指定要导入的技能名称
6. 点击"导入"

### 示例 2：导入 claudekit-skills 中的 chrome-devtools

1. 打开"技能管理"页面
2. 点击"导入技能"
3. 选择"Git 仓库"
4. 输入：
   - **Git URL**: `https://github.com/mrgoonie/claudekit-skills`
   - **子目录**: `chrome-devtools`
5. 点击"导入"

### 示例 3：导入整个技能仓库

1. 打开"技能管理"页面
2. 点击"导入技能"
3. 选择"Git 仓库"
4. 输入：
   - **Git URL**: `https://github.com/user/my-skill.git`
   - **子目录**: (留空)
5. 点击"导入"

## 📋 技能包结构

### ZIP 文件结构

**单技能 ZIP**：
```
my-skill.zip
├── SKILL.md          # 技能说明
├── prompt.md         # 提示词
└── ...
```

**多技能 ZIP**：
```
skills-collection.zip
├── skill-1/
│   ├── SKILL.md
│   └── prompt.md
├── skill-2/
│   ├── SKILL.md
│   └── prompt.md
└── skill-3/
    ├── SKILL.md
    └── prompt.md
```
导入时需要指定子目录，例如 `skill-1`。

### Git 仓库结构

### 单技能仓库

```
my-skill/
├── SKILL.md          # 技能说明
├── prompt.md         # 提示词
└── ...
```

导入时不需要指定子目录。

### 多技能仓库

```
skills-collection/
├── skill-1/
│   ├── SKILL.md
│   └── prompt.md
├── skill-2/
│   ├── SKILL.md
│   └── prompt.md
└── skill-3/
    ├── SKILL.md
    └── prompt.md
```

导入时需要指定子目录，例如 `skill-1`。

## 🔄 导入后的操作

导入成功后，技能会被存储在：
```
%USERPROFILE%\.skillmaster\skills\{skill-id}\
```

您可以：
1. 在"技能管理"页面查看已导入的技能
2. 在"AI 工具"页面同步技能到各个 AI 工具
3. 编辑或删除技能

## 💡 提示

### ZIP 文件
- 支持标准 ZIP 格式
- 文件路径可以使用正斜杠 `/` 或反斜杠 `\`
- 如果 ZIP 只有一个根目录，会自动识别并使用
- 支持包含多个技能的 ZIP 文件，通过子目录指定

### 子目录路径
- 使用正斜杠 `/` 或反斜杠 `\` 都可以
- 例如：`skills/chrome-devtools` 或 `skills\chrome-devtools`

### Git 仓库 URL
支持的格式：
- HTTPS: `https://github.com/user/repo.git`
- HTTPS (无 .git): `https://github.com/user/repo`
- SSH: `git@github.com:user/repo.git`

### 本地路径
- 绝对路径：`C:\path\to\skill.zip`
- 相对路径：`.\my-skill.zip`
- 网络路径：`\\server\share\skill.zip`

## 🐛 常见问题

### Q: 如何创建技能 ZIP 包？

**A**: 
1. 将技能文件夹压缩为 ZIP 格式
2. 确保 SKILL.md 等文件在 ZIP 的根目录或单一子目录中
3. Windows: 右键 → 发送到 → 压缩(zipped)文件夹

### Q: ZIP 导入失败，提示"无法解压"

**A**: 
- 确保 ZIP 文件没有损坏
- 检查文件路径是否正确
- 尝试重新压缩文件

### Q: 导入失败，提示"子目录不存在"

**A**: 检查子目录名称是否正确，区分大小写。

### Q: Git 克隆很慢

**A**: 
- 这取决于仓库大小和网络速度
- 建议：先在本地克隆，然后打包为 ZIP 导入

### Q: 如何导入私有仓库？

**A**: 目前需要先在本地克隆，然后打包为 ZIP 导入。

### Q: 可以同时导入多个子目录吗？

**A**: 目前不支持，需要分别导入每个子目录。

## 🚀 高级用法

### 批量导入

如果一个 ZIP 或仓库有多个技能，可以多次导入，每次指定不同的子目录：

1. 导入 `skill-1`
2. 导入 `skill-2`
3. 导入 `skill-3`

### 更新技能

如果技能有更新：
1. 重新下载或克隆最新版本
2. 打包为 ZIP 或直接从 Git 导入
3. 会创建新的技能实例（不会覆盖旧版本）

### 技能分享

要分享你的技能给其他人：
1. 找到技能目录：`%USERPROFILE%\.skillmaster\skills\{skill-id}\`
2. 将整个文件夹压缩为 ZIP
3. 分享 ZIP 文件
4. 接收者使用"ZIP 文件"方式导入

---

**提示**: 导入的技能会自动生成唯一 ID，不会覆盖已有技能。
