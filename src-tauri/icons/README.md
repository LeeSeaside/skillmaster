# 应用图标

请将应用图标放置在此目录：

- `32x32.png` - 32x32 像素
- `128x128.png` - 128x128 像素
- `128x128@2x.png` - 256x256 像素（高DPI）
- `icon.icns` - macOS 图标（如果需要跨平台）
- `icon.ico` - Windows 图标

## 临时方案

在开发阶段，Tauri 会使用默认图标。您可以稍后添加自定义图标。

## 生成图标

可以使用在线工具从单个 PNG 生成所有尺寸：
- https://icon.kitchen/
- https://www.appicon.co/

或使用 Tauri 的图标生成工具：
```bash
npm install -g @tauri-apps/cli
tauri icon path/to/icon.png
```
