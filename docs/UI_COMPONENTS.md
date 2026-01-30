# UI 组件使用指南

项目使用 **shadcn-vue** 组件库（shadcn/ui 的 Vue 3 版本）。

## 📦 核心依赖

```json
{
  "radix-vue": "^1.9.0",
  "class-variance-authority": "^0.7.0",
  "clsx": "^2.1.0",
  "tailwind-merge": "^2.2.0",
  "lucide-vue-next": "^0.344.0",
  "@vueuse/core": "^10.7.0",
  "tailwindcss-animate": "^1.0.7"
}
```

## 🎨 组件使用

### Button 组件

```vue
<script setup>
import { Button } from '@/components/ui/button'
</script>

<template>
  <Button variant="default">点击我</Button>
  <Button variant="outline" size="sm">小按钮</Button>
  <Button variant="destructive">删除</Button>
</template>
```

**变体**: default, destructive, outline, secondary, ghost, link  
**尺寸**: default, sm, lg, icon

### Card 组件

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
    <CardContent>内容</CardContent>
    <CardFooter>底部</CardFooter>
  </Card>
</template>
```

### Badge 组件

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

### Input 组件

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

## 🛠️ 工具函数

### cn() 函数

用于合并 TailwindCSS 类名：

```typescript
import { cn } from '@/lib/utils'

const className = cn(
  'base-class',
  condition && 'conditional-class',
  props.class
)
```

## 🎨 自定义主题

修改 `src/styles/index.css` 中的 CSS 变量：

```css
:root {
  --primary: 221.2 83.2% 53.3%;  /* 主色调 */
  --radius: 0.75rem;              /* 圆角大小 */
}
```

## 🚀 添加新组件

1. 访问 [shadcn-vue 文档](https://www.shadcn-vue.com/)
2. 选择需要的组件
3. 复制组件代码到 `src/components/ui/`
4. 更新导入路径为 `@/lib/utils`

## 📚 参考资源

- [shadcn-vue 官方文档](https://www.shadcn-vue.com/)
- [Radix Vue 文档](https://www.radix-vue.com/)
- [TailwindCSS 文档](https://tailwindcss.com/)

## ✨ 优势

- 基于 Radix Vue，完全无障碍
- 使用 CVA 管理样式变体
- 完整的 TypeScript 支持
- 易于定制和扩展
