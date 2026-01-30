<template>
  <div class="p-8">
    <div class="mb-8">
      <h2 class="text-3xl font-bold tracking-tight">仪表盘</h2>
      <p class="text-muted-foreground">管理您的AI技能和工具</p>
    </div>

    <!-- Stats Cards -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-8">
      <Card>
        <CardHeader>
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">总技能数</p>
            <Icon icon="mdi:puzzle" class="h-4 w-4 text-muted-foreground" />
          </div>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ skillStore.skills.length }}</div>
          <p class="text-xs text-muted-foreground">已导入的技能</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">已安装工具</p>
            <Icon icon="mdi:tools" class="h-4 w-4 text-muted-foreground" />
          </div>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ installedToolsCount }}</div>
          <p class="text-xs text-muted-foreground">检测到的AI工具</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">活跃技能</p>
            <Icon icon="mdi:check-circle" class="h-4 w-4 text-muted-foreground" />
          </div>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ activeSkillsCount }}</div>
          <p class="text-xs text-muted-foreground">已同步的技能</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">仓库路径</p>
            <Icon icon="mdi:folder" class="h-4 w-4 text-muted-foreground" />
          </div>
        </CardHeader>
        <CardContent>
          <div class="text-sm font-mono truncate">{{ repoPath }}</div>
          <p class="text-xs text-muted-foreground">中央仓库</p>
        </CardContent>
      </Card>
    </div>

    <!-- Quick Actions -->
    <Card>
      <CardHeader>
        <h3 class="text-lg font-semibold">快速操作</h3>
      </CardHeader>
      <CardContent class="flex gap-4">
        <Button @click="$router.push('/skills')">
          <Icon icon="mdi:plus" class="mr-2 h-4 w-4" />
          导入技能
        </Button>
        <Button variant="outline" @click="refreshData">
          <Icon icon="mdi:refresh" class="mr-2 h-4 w-4" />
          刷新数据
        </Button>
        <Button variant="outline" @click="$router.push('/tools')">
          <Icon icon="mdi:tools" class="mr-2 h-4 w-4" />
          管理工具
        </Button>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { Card, CardHeader, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { useSkillStore } from '@/stores/skill'
import { useToolStore } from '@/stores/tool'

const skillStore = useSkillStore()
const toolStore = useToolStore()

const installedToolsCount = computed(() => toolStore.getInstalledTools().length)
const activeSkillsCount = computed(() => skillStore.skills.filter(s => s.status === 'active').length)
const repoPath = computed(() => '%USERPROFILE%\\.skillmaster')

async function refreshData() {
  await Promise.all([
    skillStore.loadSkills(),
    toolStore.detectTools(),
  ])
}

onMounted(() => {
  refreshData()
})
</script>
