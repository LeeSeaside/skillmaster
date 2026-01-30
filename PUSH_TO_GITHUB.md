# 推送到 GitHub 指南

## 方法一：使用脚本（推荐）

### Windows

双击运行 `push-to-github.bat` 文件，或在命令行中执行：

```cmd
push-to-github.bat
```

### macOS/Linux

```bash
chmod +x push-to-github.sh
./push-to-github.sh
```

## 方法二：手动执行命令

### 1. 初始化 Git 仓库（如果还没有）

```bash
git init
```

### 2. 添加远程仓库

```bash
git remote add origin https://github.com/LeeSeaside/skillmaster.git
```

如果已经添加过，更新 URL：

```bash
git remote set-url origin https://github.com/LeeSeaside/skillmaster.git
```

### 3. 添加所有文件

```bash
git add .
```

### 4. 提交更改

```bash
git commit -m "Initial commit: SkillMaster v0.1.0"
```

### 5. 设置主分支

```bash
git branch -M main
```

### 6. 推送到 GitHub

```bash
git push -u origin main
```

## 验证

推送成功后，访问以下地址验证：

https://github.com/LeeSeaside/skillmaster

## 后续推送

首次推送后，后续更新只需：

```bash
git add .
git commit -m "你的提交信息"
git push
```

## 常见问题

### Q: 推送失败，提示认证错误？

A: 需要配置 GitHub 认证：

**使用 Personal Access Token (推荐)**

1. 在 GitHub 生成 Token: Settings → Developer settings → Personal access tokens
2. 使用 Token 作为密码推送

**使用 SSH**

```bash
# 生成 SSH 密钥
ssh-keygen -t ed25519 -C "your_email@example.com"

# 添加到 GitHub: Settings → SSH and GPG keys

# 更改远程 URL 为 SSH
git remote set-url origin git@github.com:LeeSeaside/skillmaster.git
```

### Q: 推送被拒绝，提示 non-fast-forward？

A: 远程仓库有新的提交，需要先拉取：

```bash
git pull origin main --rebase
git push origin main
```

### Q: 文件太大无法推送？

A: 检查 `.gitignore` 是否正确配置，确保不推送：
- `node_modules/`
- `src-tauri/target/`
- `dist/`
- 构建产物（.msi, .exe, .dmg 等）

### Q: 如何撤销最后一次提交？

```bash
# 撤销提交但保留更改
git reset --soft HEAD~1

# 撤销提交并丢弃更改
git reset --hard HEAD~1
```

## 推送检查清单

推送前确保：

- [ ] `.gitignore` 已正确配置
- [ ] 没有包含敏感信息（密钥、密码等）
- [ ] 没有包含大文件（> 100MB）
- [ ] 代码可以正常构建
- [ ] 提交信息清晰明确
- [ ] 已更新相关文档

## 下一步

推送成功后：

1. 在 GitHub 上查看仓库
2. 设置仓库描述和主题
3. 启用 GitHub Actions
4. 创建第一个 Release
5. 编写详细的 README

## 相关链接

- GitHub 仓库: https://github.com/LeeSeaside/skillmaster
- GitHub Actions: https://github.com/LeeSeaside/skillmaster/actions
- Releases: https://github.com/LeeSeaside/skillmaster/releases
