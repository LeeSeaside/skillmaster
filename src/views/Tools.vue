<template>
  <div class="p-8">
    <div class="mb-8 flex items-center justify-between">
      <div>
        <h2 class="text-3xl font-bold tracking-tight">AI工具管理</h2>
        <p class="text-muted-foreground">检测和管理已安装的AI编程工具</p>
      </div>
      <div class="flex gap-2">
        <Button 
          variant="outline"
          @click="showAddToolDialog = true"
        >
          <Icon icon="mdi:plus" class="mr-2 h-4 w-4" />
          添加工具
        </Button>
        <Button 
          v-if="toolStore.getInstalledTools().length > 0" 
          @click="syncAllSkillsToAllTools"
          :disabled="syncing"
        >
          <Icon v-if="syncing" icon="mdi:loading" class="mr-2 h-4 w-4 animate-spin" />
          <Icon v-else icon="mdi:sync-circle" class="mr-2 h-4 w-4" />
          {{ syncing ? '同步中...' : '一键同步所有技能' }}
        </Button>
      </div>
    </div>

    <!-- Add/Edit Tool Dialog -->
    <div v-if="showAddToolDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">{{ editingTool ? '编辑工具' : '添加自定义工具' }}</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div>
            <label class="text-sm font-medium mb-2 block">工具 ID</label>
            <Input 
              v-model="toolForm.id" 
              placeholder="例如: my_custom_tool"
              :disabled="!!editingTool"
            />
            <p class="text-xs text-muted-foreground mt-1">唯一标识符，只能包含字母、数字和下划线</p>
          </div>
          <div>
            <label class="text-sm font-medium mb-2 block">显示名称</label>
            <Input 
              v-model="toolForm.displayName" 
              placeholder="例如: My Custom Tool"
            />
          </div>
          <div>
            <label class="text-sm font-medium mb-2 block">检测目录</label>
            <Input 
              v-model="toolForm.detectDir" 
              placeholder="例如: .my-tool"
            />
            <p class="text-xs text-muted-foreground mt-1">相对于用户主目录的路径</p>
          </div>
          <div>
            <label class="text-sm font-medium mb-2 block">技能目录</label>
            <Input 
              v-model="toolForm.skillsDir" 
              placeholder="例如: .my-tool/skills"
            />
            <p class="text-xs text-muted-foreground mt-1">相对于用户主目录的路径</p>
          </div>
        </CardContent>
        <CardFooter>
          <div class="flex gap-2 w-full">
            <Button 
              variant="outline" 
              @click="closeAddToolDialog"
              class="flex-1"
            >
              取消
            </Button>
            <Button 
              @click="saveCustomTool"
              :disabled="!isToolFormValid"
              class="flex-1"
            >
              {{ editingTool ? '保存' : '添加' }}
            </Button>
          </div>
        </CardFooter>
      </Card>
    </div>

    <!-- Syncing Progress Dialog -->
    <div v-if="showSyncDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">{{ syncDialogTitle }}</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center gap-3">
            <Icon v-if="!syncComplete" icon="mdi:loading" class="h-8 w-8 animate-spin text-primary" />
            <Icon v-else icon="mdi:check-circle" class="h-8 w-8 text-green-500" />
            <div class="flex-1">
              <p class="text-sm font-medium">{{ syncDialogMessage }}</p>
              <p v-if="syncProgress" class="text-xs text-muted-foreground mt-1 whitespace-pre-line">{{ syncProgress }}</p>
            </div>
          </div>
        </CardContent>
        <CardFooter v-if="syncComplete">
          <Button @click="closeSyncDialog" class="w-full">
            关闭
          </Button>
        </CardFooter>
      </Card>
    </div>

    <div v-if="toolStore.loading" class="text-center py-12">
      <Icon icon="mdi:loading" class="h-8 w-8 animate-spin mx-auto text-primary" />
      <p class="mt-2 text-muted-foreground">检测中...</p>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2">
      <Card v-for="tool in toolStore.tools" :key="tool.id" :class="{ 'opacity-50': !tool.isInstalled }">
        <CardHeader>
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <Icon icon="mdi:tools" class="h-6 w-6" :class="tool.isInstalled ? 'text-primary' : 'text-muted-foreground'" />
              <div>
                <h3 class="font-semibold">{{ tool.displayName }}</h3>
                <p class="text-xs text-muted-foreground">{{ tool.name }}</p>
              </div>
            </div>
            <Button
              v-if="tool.isCustom"
              size="sm"
              variant="ghost"
              @click="editTool(tool)"
            >
              <Icon icon="mdi:pencil" class="h-4 w-4" />
            </Button>
          </div>
        </CardHeader>
        <CardContent class="space-y-3">
          <div>
            <p class="text-sm font-medium">技能目录</p>
            <p class="text-xs text-muted-foreground font-mono">{{ tool.skillsDir }}</p>
          </div>
          <div class="flex gap-2">
            <Button
              v-if="tool.isInstalled"
              size="sm"
              variant="outline"
              class="flex-1"
              @click="syncAllSkillsToTool(tool.id, tool.displayName)"
              :disabled="syncing"
            >
              <Icon icon="mdi:sync" class="mr-2 h-4 w-4" />
              同步所有技能
            </Button>
            <Button
              v-if="tool.isCustom"
              size="sm"
              variant="destructive"
              @click="deleteCustomTool(tool.id, tool.displayName)"
            >
              <Icon icon="mdi:delete" class="h-4 w-4" />
            </Button>
          </div>
          <div v-if="!tool.isInstalled" class="text-center py-2">
            <p class="text-xs text-muted-foreground">未检测到此工具</p>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useToolStore } from '@/stores/tool'
import { useSkillStore } from '@/stores/skill'

const toolStore = useToolStore()
const skillStore = useSkillStore()
const syncing = ref(false)
const showSyncDialog = ref(false)
const syncDialogTitle = ref('')
const syncDialogMessage = ref('')
const syncProgress = ref('')
const syncComplete = ref(false)

// Add/Edit Tool Dialog
const showAddToolDialog = ref(false)
const editingTool = ref<any>(null)
const toolForm = ref({
  id: '',
  displayName: '',
  detectDir: '',
  skillsDir: '',
})

const isToolFormValid = computed(() => {
  return toolForm.value.id && 
         toolForm.value.displayName && 
         toolForm.value.detectDir && 
         toolForm.value.skillsDir &&
         /^[a-z0-9_]+$/.test(toolForm.value.id)
})

function closeAddToolDialog() {
  showAddToolDialog.value = false
  editingTool.value = null
  toolForm.value = {
    id: '',
    displayName: '',
    detectDir: '',
    skillsDir: '',
  }
}

function editTool(tool: any) {
  editingTool.value = tool
  toolForm.value = {
    id: tool.id,
    displayName: tool.displayName,
    detectDir: tool.detectDir,
    skillsDir: tool.skillsDir,
  }
  showAddToolDialog.value = true
}

function saveCustomTool() {
  if (!isToolFormValid.value) return

  if (editingTool.value) {
    // Update existing tool
    toolStore.updateCustomTool(toolForm.value.id, {
      displayName: toolForm.value.displayName,
      detectDir: toolForm.value.detectDir,
      skillsDir: toolForm.value.skillsDir,
    })
  } else {
    // Add new tool
    toolStore.addCustomTool({
      id: toolForm.value.id,
      name: toolForm.value.id,
      displayName: toolForm.value.displayName,
      detectDir: toolForm.value.detectDir,
      skillsDir: toolForm.value.skillsDir,
    })
  }

  closeAddToolDialog()
  toolStore.detectTools()
}

function deleteCustomTool(toolId: string, toolName: string) {
  const confirmed = confirm(
    `确定要从列表中移除自定义工具"${toolName}"吗？\n\n` +
    `注意：这只会从 SkillMaster 中移除该工具配置，\n` +
    `不会删除工具本身的文件夹或配置。`
  )
  if (!confirmed) return

  toolStore.deleteCustomTool(toolId)
  toolStore.detectTools()
}

function closeSyncDialog() {
  showSyncDialog.value = false
  syncDialogTitle.value = ''
  syncDialogMessage.value = ''
  syncProgress.value = ''
  syncComplete.value = false
}

async function syncAllSkillsToTool(toolId: string, toolName: string) {
  if (skillStore.skills.length === 0) {
    alert('暂无技能可同步。请先导入技能。')
    return
  }

  syncing.value = true
  showSyncDialog.value = true
  syncDialogTitle.value = '同步技能'
  syncDialogMessage.value = `正在同步到 ${toolName}...`
  syncProgress.value = ''
  syncComplete.value = false
  
  try {
    let successCount = 0
    let failedCount = 0
    const total = skillStore.skills.length
    const errors: string[] = []
    
    for (let i = 0; i < skillStore.skills.length; i++) {
      const skill = skillStore.skills[i]
      syncProgress.value = `进度: ${i + 1}/${total} - ${skill.name}`
      
      try {
        await skillStore.syncSkill(skill.id, toolId, true)
        successCount++
      } catch (e) {
        failedCount++
        const errorMsg = String(e)
        console.error(`Failed to sync ${skill.name} to ${toolName}:`, errorMsg)
        
        // Collect first few errors for display
        if (errors.length < 3) {
          errors.push(`${skill.name}: ${errorMsg}`)
        }
      }
    }
    
    syncComplete.value = true
    syncDialogTitle.value = '同步完成'
    syncDialogMessage.value = `已同步到 ${toolName}`
    
    let progressText = `成功: ${successCount} 个技能`
    if (failedCount > 0) {
      progressText += ` | 失败: ${failedCount} 个技能`
      if (errors.length > 0) {
        progressText += `\n\n错误详情:\n${errors.join('\n')}`
      }
    }
    syncProgress.value = progressText
  } catch (e) {
    syncComplete.value = true
    syncDialogTitle.value = '同步失败'
    syncDialogMessage.value = String(e)
    syncProgress.value = ''
  } finally {
    syncing.value = false
  }
}

async function syncAllSkillsToAllTools() {
  if (skillStore.skills.length === 0) {
    alert('暂无技能可同步。请先导入技能。')
    return
  }

  const installedTools = toolStore.getInstalledTools()
  if (installedTools.length === 0) {
    alert('未检测到已安装的 AI 工具。')
    return
  }

  const confirmed = confirm(
    `确定要将所有 ${skillStore.skills.length} 个技能同步到 ${installedTools.length} 个 AI 工具吗？\n\n` +
    `工具列表：\n${installedTools.map(t => `• ${t.displayName}`).join('\n')}`
  )

  if (!confirmed) return

  syncing.value = true
  showSyncDialog.value = true
  syncDialogTitle.value = '批量同步'
  syncDialogMessage.value = '正在同步所有技能到所有工具...'
  syncProgress.value = ''
  syncComplete.value = false
  
  try {
    let totalSuccess = 0
    let totalFailed = 0
    const totalOperations = installedTools.length * skillStore.skills.length
    let currentOperation = 0
    
    for (const tool of installedTools) {
      syncDialogMessage.value = `正在同步到 ${tool.displayName}...`
      
      for (const skill of skillStore.skills) {
        currentOperation++
        syncProgress.value = `进度: ${currentOperation}/${totalOperations} - ${skill.name} → ${tool.displayName}`
        
        try {
          await skillStore.syncSkill(skill.id, tool.id, true)
          totalSuccess++
        } catch (e) {
          totalFailed++
          console.error(`Failed to sync ${skill.name} to ${tool.displayName}:`, e)
        }
      }
    }
    
    syncComplete.value = true
    syncDialogTitle.value = '批量同步完成'
    syncDialogMessage.value = `已将 ${skillStore.skills.length} 个技能同步到 ${installedTools.length} 个工具`
    syncProgress.value = `成功: ${totalSuccess} 次同步${totalFailed > 0 ? ` | 失败: ${totalFailed} 次同步` : ''}`
  } catch (e) {
    syncComplete.value = true
    syncDialogTitle.value = '批量同步失败'
    syncDialogMessage.value = String(e)
    syncProgress.value = ''
  } finally {
    syncing.value = false
  }
}

onMounted(() => {
  toolStore.detectTools()
  skillStore.loadSkills()
})
</script>
