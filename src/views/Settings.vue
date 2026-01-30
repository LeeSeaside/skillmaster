<template>
  <div class="p-8">
    <div class="mb-8">
      <h2 class="text-3xl font-bold tracking-tight">设置</h2>
      <p class="text-muted-foreground">配置应用程序选项</p>
    </div>

    <div class="max-w-2xl space-y-6">
      <Card>
        <CardHeader>
          <h3 class="text-lg font-semibold">仓库设置</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div>
            <label class="text-sm font-medium mb-2 block">中央仓库路径</label>
            <Input v-model="repoPath" placeholder="%USERPROFILE%\.skillmaster" disabled />
            <p class="text-xs text-muted-foreground mt-1">技能的中央存储位置</p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <h3 class="text-lg font-semibold">同步设置</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium">优先使用符号链接</p>
              <p class="text-xs text-muted-foreground">使用 junction 而不是复制文件</p>
            </div>
            <input
              type="checkbox"
              v-model="useSymlink"
              class="h-4 w-4 rounded border-gray-300"
            />
          </div>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium">非破坏性同步</p>
              <p class="text-xs text-muted-foreground">不覆盖已存在的目标文件夹</p>
            </div>
            <input
              type="checkbox"
              v-model="nonDestructive"
              class="h-4 w-4 rounded border-gray-300"
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <h3 class="text-lg font-semibold">关于</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex justify-between">
            <span class="text-sm font-medium">版本</span>
            <span class="text-sm text-muted-foreground">{{ appVersion }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm font-medium">构建</span>
            <span class="text-sm text-muted-foreground">Tauri 2.0 + Vue 3</span>
          </div>
          <div class="pt-2">
            <Button 
              @click="checkForUpdates" 
              :disabled="checking"
              class="w-full"
            >
              <Icon v-if="checking" icon="mdi:loading" class="mr-2 h-4 w-4 animate-spin" />
              <Icon v-else icon="mdi:update" class="mr-2 h-4 w-4" />
              {{ checking ? '检查中...' : '检查更新' }}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Update Dialog -->
    <div v-if="showUpdateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">{{ updateDialogTitle }}</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-start gap-3">
            <Icon 
              :icon="updateDialogIcon" 
              :class="updateDialogIconClass"
              class="h-5 w-5 flex-shrink-0 mt-0.5" 
            />
            <div class="flex-1">
              <p class="text-sm whitespace-pre-line">{{ updateDialogMessage }}</p>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <Button @click="closeUpdateDialog" class="w-full">
            确定
          </Button>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { getVersion } from '@tauri-apps/api/app'

const repoPath = ref('%USERPROFILE%\\.skillmaster')
const useSymlink = ref(true)
const nonDestructive = ref(true)
const appVersion = ref('0.1.0')
const checking = ref(false)
const showUpdateDialog = ref(false)
const updateDialogTitle = ref('')
const updateDialogMessage = ref('')
const updateDialogIcon = ref('mdi:information-outline')
const updateDialogIconClass = ref('text-primary')

onMounted(async () => {
  try {
    appVersion.value = await getVersion()
  } catch (e) {
    console.error('Failed to get app version:', e)
  }
})

async function checkForUpdates() {
  checking.value = true
  
  try {
    const update = await check()
    
    if (update?.available) {
      updateDialogTitle.value = '发现新版本'
      updateDialogMessage.value = `发现新版本 ${update.version}！\n\n当前版本: ${appVersion.value}\n最新版本: ${update.version}\n\n是否立即下载并安装？`
      updateDialogIcon.value = 'mdi:update'
      updateDialogIconClass.value = 'text-primary'
      showUpdateDialog.value = true
      
      // 下载并安装更新
      try {
        await update.downloadAndInstall()
        
        updateDialogTitle.value = '更新完成'
        updateDialogMessage.value = '更新已下载并安装完成！\n\n应用将重启以应用更新。'
        updateDialogIcon.value = 'mdi:check-circle'
        updateDialogIconClass.value = 'text-green-500'
        
        // 延迟重启
        setTimeout(async () => {
          await relaunch()
        }, 2000)
      } catch (e) {
        updateDialogTitle.value = '更新失败'
        updateDialogMessage.value = `更新下载或安装失败：\n\n${e}`
        updateDialogIcon.value = 'mdi:alert-circle'
        updateDialogIconClass.value = 'text-destructive'
      }
    } else {
      updateDialogTitle.value = '已是最新版本'
      updateDialogMessage.value = `当前版本 ${appVersion.value} 已是最新版本。`
      updateDialogIcon.value = 'mdi:check-circle'
      updateDialogIconClass.value = 'text-green-500'
      showUpdateDialog.value = true
    }
  } catch (e) {
    updateDialogTitle.value = '检查更新失败'
    updateDialogMessage.value = `无法检查更新：\n\n${e}\n\n请检查网络连接或稍后再试。`
    updateDialogIcon.value = 'mdi:alert-circle'
    updateDialogIconClass.value = 'text-destructive'
    showUpdateDialog.value = true
  } finally {
    checking.value = false
  }
}

function closeUpdateDialog() {
  showUpdateDialog.value = false
}
</script>
