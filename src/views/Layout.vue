<template>
  <div class="flex h-screen bg-background">
    <!-- Sidebar -->
    <aside class="w-64 border-r bg-card">
      <div class="flex h-16 items-center border-b px-6">
        <h1 class="text-xl font-bold text-primary">SkillMaster</h1>
      </div>
      <nav class="space-y-1 p-4">
        <router-link
          v-for="item in menuItems"
          :key="item.path"
          :to="item.path"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
          active-class="bg-accent text-accent-foreground"
        >
          <Icon :icon="item.icon" class="h-5 w-5" />
          <span>{{ item.label }}</span>
        </router-link>
      </nav>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 overflow-auto">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { Icon } from '@iconify/vue'

const menuItems = [
  { path: '/dashboard', label: '仪表盘', icon: 'mdi:view-dashboard' },
  { path: '/skills', label: '技能管理', icon: 'mdi:puzzle' },
  { path: '/tools', label: 'AI工具', icon: 'mdi:tools' },
  { path: '/settings', label: '设置', icon: 'mdi:cog' },
]
</script>
