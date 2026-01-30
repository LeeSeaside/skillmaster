# SkillMaster 开发文档

## 📋 目录

- [快速开始](#快速开始)
- [项目架构](#项目架构)
- [技术栈](#技术栈)
- [开发指南](#开发指南)
- [shadcn-vue 使用](#shadcn-vue-使用)
- [常见问题](#常见问题)

---

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust (稳定版本)
- Windows 10/11

### 安装步骤

```bash
# 1. 安装 Rust（如果还没安装）
winget install Rustlang.Rustup

# 2. 重启终端，安装依赖
npm install

# 3. 启动开发服务器
npm run tauri:dev
```

### 常用命令

```bash
# 开发模式（热重载）
npm run tauri:dev

# 构建生产版本
npm run tauri:build

# 仅运行前端
npm run dev

# 构建前端
npm run build
```

---

## 🏗️ 项目架构

### 目录结构

```
skillmaster/
├── src/                      # Vue 前端
│   ├── components/ui/       # shadcn-vue 组件
│   ├── views/              # 页面组件
│   ├── stores/             # Pinia 状态管理
│   ├── router/             # Vue Router
│   ├── types/              # TypeScript 类型
│   ├── lib/                # 工具函数
│   └── styles/             # 全局样式
├── src-tauri/               # Rust 后端
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── models/         # 数据模型
│   │   ├── services/       # 业务逻辑
│   │   └── utils/          # 工具函数
│   └── Cargo.toml          # Rust 依赖
└── docs/                    # 项目文档
```

### 核心功能

- **技能仓库管理** - 中央仓库 `%USERPROFILE%\.skillmaster`
- **技能导入** - 支持本地文件夹和 Git 仓库
- **AI 工具检测** - 自动检测 Cursor、Claude Code、Windsurf、GitHub Copilot
- **智能同步** - Junction 优先，复制回退

---

## 💻 技术栈

### 前端

```
Vue 3.4.0 + TypeScript 5.3.0
├── shadcn-vue (Radix Vue + CVA)
├── TailwindCSS 3.4.0
├── Vue Router 4.2.5
├── Pinia 2.1.7
└── Iconify 4.1.1
```

### 后端

```
Tauri 2.0 + Rust
├── serde (序列化)
├── git2 (Git 操作)
├── uuid (ID 生成)
├── chrono (时间)
└── dirs (路径)
```

---

## 🛠️ 开发指南

### 添加新页面

1. 在 `src/views/` 创建 Vue 组件
2. 在 `src/router/index.ts` 添加路由
3. 在 `src/views/Layout.vue` 添加菜单项

```vue
<!-- src/views/NewPage.vue -->
<template>
  <div class="p-8">
    <h2 class="text-3xl font-bold">新页面</h2>
  </div>
</template>
```

### 添加 Tauri 命令

1. 在 `src-tauri/src/commands/` 创建命令函数

```rust
// src-tauri/src/commands/my_commands.rs
#[tauri::command]
pub fn my_command(arg: String) -> Result<String, String> {
    Ok(format!("Received: {}", arg))
}
```

2. 在 `src-tauri/src/main.rs` 注册命令

```rust
.invoke_handler(tauri::generate_handler![
    my_command,
    // ... 其他命令
])
```

3. 在前端调用

```typescript
import { invoke } from '@tauri-apps/api/core'

const result = await invoke<string>('my_command', { arg: 'value' })
```

### 状态管理

使用 Pinia 管理全局状态：

```typescript
// src/stores/myStore.ts
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useMyStore = defineStore('my', () => {
  const data = ref<string[]>([])
  
  async function loadData() {
    data.value = await invoke<string[]>('get_data')
  }
  
  return { data, loadData }
})
```

---

## 🎨 shadcn-vue 使用

### 什么是 shadcn-vue？

shadcn-vue 不是传统的 npm 包，而是**复制粘贴式的组件集合**。组件代码在你的项目中（`src/components/ui/`），可以随意修改。

### 已有组件

#### Button 组件

```vue
<script setup>
import { Button } from '@/components/ui/button'
</script>

<template>
  <Button variant="default">默认按钮</Button>
  <Button variant="outline" size="sm">小按钮</Button>
  <Button variant="destructive">删除</Button>
</template>
```

**变体**: default, destructive, outline, secondary, ghost, link  
**尺寸**: default, sm, lg, icon

#### Card 组件

```vue
<script setup>
import { 
  Card, 
  CardHeader, 
  CardTitle, 
  CardContent, 
  CardFooter 
} from '@/components/ui/card'
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>标题</CardTitle>
    </CardHeader>
    <CardContent>
      内容
    </CardContent>
    <CardFooter>
      底部
    </CardFooter>
  </Card>
</template>
```

#### Badge 组件

```vue
<script setup>
import { Badge } from '@/components/ui/badge'
</script>

<template>
  <Badge variant="default">默认</Badge>
  <Badge variant="success">成功</Badge>
  <Badge variant="destructive">错误</Badge>
</template>
```

#### Input 组件

```vue
<script setup>
import { ref } from 'vue'
import { Input } from '@/components/ui/input'

const value = ref('')
</script>

<template>
  <Input v-model="value" placeholder="请输入..." />
</template>
```

### cn() 工具函数

用于智能合并 TailwindCSS 类名：

```typescript
import { cn } from '@/lib/utils'

const className = cn(
  'base-class',
  condition && 'conditional-class',
  props.class
)
```

### 添加更多组件

访问 [shadcn-vue.com](https://www.shadcn-vue.com/) 复制需要的组件代码到 `src/components/ui/`

---

## 🐛 常见问题

### Q: Rust 编译时间很长

**A**: 首次编译需要 3-5 分钟，这是正常的。后续编译会快很多。

### Q: 端口 1420 被占用

**A**: 修改 `vite.config.ts` 中的 `server.port` 配置。

### Q: 修改 Rust 代码后没有生效

**A**: Tauri 会自动检测并重新编译，等待编译完成即可。

### Q: TypeScript 类型错误

**A**: 运行 `npm run build` 检查是否有真实错误。

### Q: 如何调试？

**A**: 
- 前端：按 F12 打开 DevTools
- 后端：查看终端输出

### Q: 如何自定义主题？

**A**: 修改 `src/styles/index.css` 中的 CSS 变量：

```css
:root {
  --primary: 221.2 83.2% 53.3%;  /* 主色调 */
  --radius: 0.75rem;              /* 圆角 */
}
```

---

## 📚 参考资源

- [Tauri 官方文档](https://tauri.app/)
- [Vue 3 官方文档](https://vuejs.org/)
- [shadcn-vue 文档](https://www.shadcn-vue.com/)
- [TailwindCSS 文档](https://tailwindcss.com/)
- [Rust 官方文档](https://doc.rust-lang.org/)

---

## 🎯 开发流程

### 1. 功能开发

```bash
# 创建功能分支
git checkout -b feature/new-feature

# 开发
npm run tauri:dev

# 提交
git add .
git commit -m "feat: add new feature"
```

### 2. 代码规范

- TypeScript: 使用严格模式
- Vue: Composition API + `<script setup>`
- Rust: 遵循官方风格指南
- 提交信息: 使用 Conventional Commits

### 3. 测试

```bash
# 构建测试
npm run tauri:build

# 运行生成的可执行文件
./src-tauri/target/release/skillmaster.exe
```

---

**最后更新**: 2026-01-29
