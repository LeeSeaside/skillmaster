# 安装指南

## 环境要求

- **Node.js**: 18+ (推荐 20+)
- **Rust**: 稳定版本
- **操作系统**: Windows 10/11

## 安装步骤

### 1. 安装 Rust

#### 方法 1: 使用 winget（推荐）

```powershell
winget install Rustlang.Rustup
```

#### 方法 2: 手动安装

访问 https://rustup.rs/ 下载并运行安装程序。

**重要**: 安装完成后必须重启终端！

#### 验证安装

```bash
rustc --version
cargo --version
```

### 2. 安装项目依赖

```bash
npm install
```

### 3. 启动应用

```bash
npm run tauri:dev
```

首次运行会编译 Rust 依赖，需要 3-5 分钟。

## 构建生产版本

```bash
npm run tauri:build
```

可执行文件位于: `src-tauri/target/release/skillmaster.exe`

## 常见问题

### Rust 未找到

确保已安装 Rust 并重启终端。

### 编译时间长

首次编译需要 3-5 分钟，这是正常的。

### 端口被占用

修改 `vite.config.ts` 中的 `server.port`。

### WebView2 缺失

Windows 10/11 通常已预装。如缺失，访问:  
https://developer.microsoft.com/microsoft-edge/webview2/

## 需要帮助？

查看 [开发文档](DEVELOPMENT.md) 或提交 Issue。
