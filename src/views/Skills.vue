<template>
  <div class="p-8">
    <div class="mb-8 flex items-center justify-between">
      <div>
        <h2 class="text-3xl font-bold tracking-tight">技能管理</h2>
        <p class="text-muted-foreground">导入和管理您的AI技能</p>
      </div>
      <Button @click="showImportDialog = true">
        <Icon icon="mdi:plus" class="mr-2 h-4 w-4" />
        导入技能
      </Button>
    </div>

    <!-- Skills List -->
    <div v-if="skillStore.loading" class="text-center py-12">
      <Icon icon="mdi:loading" class="h-8 w-8 animate-spin mx-auto text-primary" />
      <p class="mt-2 text-muted-foreground">加载中...</p>
    </div>

    <div v-else-if="skillStore.skills.length === 0" class="text-center py-12">
      <Icon icon="mdi:puzzle-outline" class="h-16 w-16 mx-auto text-muted-foreground" />
      <h3 class="mt-4 text-lg font-semibold">暂无技能</h3>
      <p class="text-muted-foreground">点击上方按钮导入您的第一个技能</p>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <Card v-for="skill in skillStore.skills" :key="skill.id" class="flex flex-col">
        <CardHeader>
          <div class="flex items-start justify-between">
            <div>
              <h3 class="font-semibold">{{ skill.name }}</h3>
              <p class="text-xs text-muted-foreground">{{ skill.version }}</p>
            </div>
            <Badge :variant="skill.status === 'active' ? 'success' : 'secondary'">
              {{ skill.status }}
            </Badge>
          </div>
        </CardHeader>
        <CardContent class="flex-1">
          <p class="text-sm text-muted-foreground mb-4">{{ skill.description }}</p>
          <div class="flex flex-wrap gap-1">
            <Badge v-for="tag in skill.tags" :key="tag" variant="outline">
              {{ tag }}
            </Badge>
          </div>
        </CardContent>
        <CardFooter class="pt-0">
          <div class="flex gap-2 w-full">
            <Button size="sm" variant="outline" @click="viewSkill(skill.id)" class="flex-1">
              查看详情
            </Button>
            <Button size="sm" variant="destructive" @click="confirmDelete(skill.id, skill.name)" class="flex-1">
              删除
            </Button>
          </div>
        </CardFooter>
      </Card>
    </div>

    <!-- Import Dialog -->
    <div v-if="showImportDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">导入技能</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- Error Message -->
          <div v-if="errorMessage" class="p-3 rounded-md bg-destructive/10 border border-destructive/20">
            <div class="flex items-start gap-2">
              <Icon icon="mdi:alert-circle" class="h-5 w-5 text-destructive flex-shrink-0 mt-0.5" />
              <div class="flex-1">
                <p class="text-sm font-medium text-destructive">导入失败</p>
                <p class="text-xs text-destructive/80 mt-1">{{ errorMessage }}</p>
              </div>
            </div>
          </div>

          <div>
            <label class="text-sm font-medium mb-2 block">来源类型</label>
            <select
              v-model="importType"
              :disabled="importing"
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm disabled:opacity-50"
            >
              <option value="zip">ZIP 文件</option>
              <option value="git">Git 仓库</option>
            </select>
          </div>
          
          <!-- ZIP File Upload -->
          <div v-if="importType === 'zip'">
            <label class="text-sm font-medium mb-2 block">选择 ZIP 文件</label>
            <div class="space-y-2">
              <!-- Drag and Drop Area -->
              <div
                @drop="handleDrop"
                @dragover="handleDragOver"
                @dragleave="handleDragLeave"
                :class="[
                  'border-2 border-dashed rounded-lg p-8 text-center transition-colors',
                  isDragging ? 'border-primary bg-primary/5' : 'border-muted-foreground/25',
                  importing ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-primary/50'
                ]"
                @click="!importing && triggerFileSelect()"
              >
                <Icon 
                  :icon="isDragging ? 'mdi:file-download' : 'mdi:cloud-upload'" 
                  class="h-12 w-12 mx-auto mb-3"
                  :class="isDragging ? 'text-primary' : 'text-muted-foreground'"
                />
                <p class="text-sm font-medium mb-1">
                  {{ isDragging ? '松开以上传文件' : '拖拽 ZIP 文件到这里' }}
                </p>
                <p class="text-xs text-muted-foreground">
                  或点击选择文件
                </p>
                <p v-if="selectedFileName" class="text-xs text-primary mt-2 font-medium">
                  已选择: {{ selectedFileName }}
                </p>
              </div>
            </div>
          </div>
          
          <!-- Git URL -->
          <div v-if="importType === 'git'">
            <label class="text-sm font-medium mb-2 block">Git URL</label>
            <Input 
              v-model="importSource" 
              :disabled="importing"
              placeholder="https://github.com/user/repo.git"
            />
            <p class="text-xs text-muted-foreground mt-1">
              💡 提示：如果克隆超时，可以先在本地克隆仓库，然后打包为 ZIP 导入
            </p>
          </div>
          <!-- Subdirectory (Optional, only show if needed) -->
          <div v-if="importType === 'git' || (importType === 'zip' && showAdvancedOptions)">
            <label class="text-sm font-medium mb-2 block">
              子目录（可选）
            </label>
            <Input 
              v-model="importSubdir" 
              :disabled="importing"
              placeholder="例如: chrome-devtools"
            />
            <p class="text-xs text-muted-foreground mt-1">
              如果 {{ importType === 'zip' ? 'ZIP 文件' : '仓库' }}包含多个技能，指定要导入的子目录
            </p>
          </div>
          
          <!-- Advanced Options Toggle for ZIP -->
          <div v-if="importType === 'zip' && !showAdvancedOptions">
            <Button
              type="button"
              variant="ghost"
              size="sm"
              @click="showAdvancedOptions = true"
              class="text-xs"
            >
              <Icon icon="mdi:chevron-down" class="mr-1 h-3 w-3" />
              高级选项
            </Button>
          </div>
        </CardContent>
        <CardFooter>
          <div class="flex gap-2 w-full">
            <Button 
              variant="outline" 
              @click="closeImportDialog" 
              :disabled="importing"
              class="flex-1"
            >
              取消
            </Button>
            <Button 
              @click="handleImport" 
              :disabled="(importType === 'zip' && !importSource) || (importType === 'git' && !importSource) || importing" 
              class="flex-1"
            >
              <Icon v-if="importing" icon="mdi:loading" class="mr-2 h-4 w-4 animate-spin" />
              {{ importing ? '导入中...' : '导入' }}
            </Button>
          </div>
        </CardFooter>
      </Card>
    </div>

    <!-- Skill Detail Dialog -->
    <div v-if="showDetailDialog && selectedSkill" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-2xl max-h-[80vh] overflow-auto">
        <CardHeader>
          <div class="flex items-start justify-between">
            <div>
              <h3 class="text-xl font-semibold">{{ selectedSkill.name }}</h3>
              <p class="text-sm text-muted-foreground mt-1">版本 {{ selectedSkill.version }}</p>
            </div>
            <Badge :variant="selectedSkill.status === 'active' ? 'success' : 'secondary'">
              {{ selectedSkill.status }}
            </Badge>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- Description -->
          <div>
            <h4 class="text-sm font-semibold mb-2">描述</h4>
            <div class="max-h-64 overflow-y-auto bg-muted p-3 rounded-md">
              <pre class="text-xs whitespace-pre-wrap font-sans text-muted-foreground">{{ selectedSkill.manifest?.skill_md_content || selectedSkill.description }}</pre>
            </div>
          </div>

          <!-- Source -->
          <div>
            <h4 class="text-sm font-semibold mb-2">来源</h4>
            <div class="flex items-center gap-2">
              <Badge variant="outline">{{ selectedSkill.source_type }}</Badge>
              <code class="text-xs bg-muted px-2 py-1 rounded">{{ selectedSkill.source }}</code>
            </div>
          </div>

          <!-- Tags -->
          <div v-if="selectedSkill.tags && selectedSkill.tags.length > 0">
            <h4 class="text-sm font-semibold mb-2">标签</h4>
            <div class="flex flex-wrap gap-1">
              <Badge v-for="tag in selectedSkill.tags" :key="tag" variant="outline">
                {{ tag }}
              </Badge>
            </div>
          </div>

          <!-- Dates -->
          <div class="grid grid-cols-2 gap-4">
            <div>
              <h4 class="text-sm font-semibold mb-1">创建时间</h4>
              <p class="text-xs text-muted-foreground">{{ formatDate(selectedSkill.created_at) }}</p>
            </div>
            <div>
              <h4 class="text-sm font-semibold mb-1">更新时间</h4>
              <p class="text-xs text-muted-foreground">{{ formatDate(selectedSkill.updated_at) }}</p>
            </div>
          </div>

          <!-- Directory Tree -->
          <div v-if="selectedSkill.directory_tree">
            <h4 class="text-sm font-semibold mb-2">清单信息</h4>
            <div class="bg-muted p-3 rounded-md overflow-x-auto">
              <pre class="text-xs font-mono">{{ selectedSkill.directory_tree }}</pre>
            </div>
          </div>

          <!-- Manifest Info (if no directory tree) -->
          <div v-else-if="selectedSkill.manifest">
            <h4 class="text-sm font-semibold mb-2">清单信息</h4>
            <div class="bg-muted p-3 rounded-md space-y-2">
              <div v-if="selectedSkill.manifest.author">
                <span class="text-xs font-medium">作者：</span>
                <span class="text-xs text-muted-foreground">{{ selectedSkill.manifest.author }}</span>
              </div>
              <div>
                <span class="text-xs font-medium">描述：</span>
                <span class="text-xs text-muted-foreground">{{ selectedSkill.manifest.description }}</span>
              </div>
            </div>
          </div>

          <!-- Sync Status -->
          <div v-if="selectedSkill.tools && selectedSkill.tools.length > 0">
            <h4 class="text-sm font-semibold mb-2">同步状态</h4>
            <div class="space-y-2">
              <div v-for="tool in selectedSkill.tools" :key="tool.tool_id" 
                   class="flex items-center justify-between p-2 bg-muted rounded-md">
                <div class="flex items-center gap-2">
                  <Icon icon="mdi:tools" class="h-4 w-4" />
                  <span class="text-sm">{{ tool.tool_name }}</span>
                </div>
                <Badge :variant="tool.sync_status === 'synced' ? 'success' : 'secondary'">
                  {{ tool.sync_status }}
                </Badge>
              </div>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <Button @click="closeDetailDialog" class="w-full">
            关闭
          </Button>
        </CardFooter>
      </Card>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteDialog && skillToDelete" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">确认删除</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-start gap-3">
            <Icon icon="mdi:information-outline" class="h-5 w-5 text-primary flex-shrink-0 mt-0.5" />
            <div class="flex-1">
              <p class="text-sm font-medium">技能 "{{ skillToDelete.name }}" 已删除</p>
              <p class="text-xs text-muted-foreground mt-1">已从中央仓库和所有 AI 工具中移除。</p>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <div class="flex gap-2 w-full">
            <Button 
              variant="outline" 
              @click="cancelDelete" 
              :disabled="deleting"
              class="flex-1"
            >
              取消
            </Button>
            <Button 
              variant="destructive" 
              @click="confirmDeleteSkill" 
              :disabled="deleting"
              class="flex-1"
            >
              <Icon v-if="deleting" icon="mdi:loading" class="mr-2 h-4 w-4 animate-spin" />
              {{ deleting ? '删除中...' : '确定' }}
            </Button>
          </div>
        </CardFooter>
      </Card>
    </div>

    <!-- Success Dialog -->
    <div v-if="showSuccessDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <Card class="w-full max-w-md">
        <CardHeader>
          <h3 class="text-lg font-semibold">SkillMaster</h3>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-start gap-3">
            <Icon icon="mdi:check-circle" class="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" />
            <div class="flex-1">
              <p class="text-sm whitespace-pre-line">{{ successMessage }}</p>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <Button @click="showSuccessDialog = false" class="w-full">
            确定
          </Button>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { useSkillStore } from '@/stores/skill'

const skillStore = useSkillStore()

const showImportDialog = ref(false)
const showDetailDialog = ref(false)
const showDeleteDialog = ref(false)
const showSuccessDialog = ref(false)
const successMessage = ref('')
const selectedSkill = ref<any>(null)
const skillToDelete = ref<{ id: string; name: string } | null>(null)
const importType = ref<'zip' | 'git'>('zip')
const importSource = ref('')
const importSubdir = ref('')
const importing = ref(false)
const deleting = ref(false)
const errorMessage = ref('')
const selectedFileName = ref('')
const isDragging = ref(false)
const showAdvancedOptions = ref(false)

let unlistenFileDrop: UnlistenFn | null = null
let unlistenFileDropHover: UnlistenFn | null = null
let unlistenFileDropCancelled: UnlistenFn | null = null

async function handleImport() {
  if (importType.value === 'zip' && !selectedFileName.value) {
    errorMessage.value = '请选择 ZIP 文件'
    return
  }
  
  if (importType.value === 'git' && !importSource.value.trim()) {
    errorMessage.value = '请输入 Git URL'
    return
  }

  importing.value = true
  errorMessage.value = ''
  
  try {
    const source = importType.value === 'zip' ? importSource.value : importSource.value.trim()
    
    console.log('Starting import:', {
      source,
      sourceType: importType.value,
      subdir: importSubdir.value.trim() || null,
    })
    
    // Add timeout to prevent hanging (5 minutes for large repositories)
    const importPromise = invoke('import_skill_with_subdir', {
      source,
      sourceType: importType.value,
      subdir: importSubdir.value.trim() || null,
    })
    
    const timeoutPromise = new Promise((_, reject) => 
      setTimeout(() => reject(new Error('导入超时（5分钟）。大型仓库可能需要更长时间。')), 300000)
    )
    
    await Promise.race([importPromise, timeoutPromise])
    
    console.log('Import successful')
    
    // Success
    closeImportDialog()
    await skillStore.loadSkills()
    
    // Show success dialog
    successMessage.value = '技能导入成功！\n\n请前往"AI工具"页面同步到各个工具。'
    showSuccessDialog.value = true
  } catch (e) {
    // Show detailed error in the dialog
    const errorMsg = String(e)
    console.error('Import failed:', errorMsg)
    errorMessage.value = errorMsg
  } finally {
    importing.value = false
  }
}

async function handleDrop(event: DragEvent) {
  // This is for browser drag and drop (fallback)
  // Tauri events will handle the actual file drop
  event.preventDefault()
  event.stopPropagation()
  isDragging.value = false
}

function handleDragOver(event: DragEvent) {
  // Keep for visual feedback
  event.preventDefault()
  event.stopPropagation()
}

function handleDragLeave(event: DragEvent) {
  // Keep for visual feedback
  event.preventDefault()
  event.stopPropagation()
}

async function handleFileSelect() {
  try {
    // Use Tauri dialog to select file
    const { open } = await import('@tauri-apps/plugin-dialog')
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'ZIP Files',
        extensions: ['zip']
      }]
    })
    
    if (selected && typeof selected === 'string') {
      importSource.value = selected
      selectedFileName.value = selected.split(/[/\\]/).pop() || selected
      errorMessage.value = ''
    }
  } catch (e) {
    console.error('Failed to select file:', e)
    errorMessage.value = '文件选择失败: ' + e
  }
}

function triggerFileSelect() {
  handleFileSelect()
}

function closeImportDialog() {
  showImportDialog.value = false
  importSource.value = ''
  importSubdir.value = ''
  selectedFileName.value = ''
  errorMessage.value = ''
  importing.value = false
  isDragging.value = false
  showAdvancedOptions.value = false
}

function confirmDelete(skillId: string, skillName: string) {
  skillToDelete.value = { id: skillId, name: skillName }
  showDeleteDialog.value = true
}

function cancelDelete() {
  showDeleteDialog.value = false
  skillToDelete.value = null
  deleting.value = false
}

async function confirmDeleteSkill() {
  if (!skillToDelete.value) return
  
  deleting.value = true
  
  try {
    await skillStore.deleteSkill(skillToDelete.value.id)
    showDeleteDialog.value = false
    const skillName = skillToDelete.value.name
    skillToDelete.value = null
    
    // Show success dialog
    successMessage.value = `技能 "${skillName}" 已删除\n\n已从中央仓库和所有 AI 工具中移除。`
    showSuccessDialog.value = true
  } catch (e) {
    alert('❌ 删除失败: ' + e)
  } finally {
    deleting.value = false
  }
}

async function viewSkill(skillId: string) {
  try {
    const detail = await skillStore.getSkillDetail(skillId)
    selectedSkill.value = detail
    showDetailDialog.value = true
  } catch (e) {
    alert('❌ 获取技能详情失败: ' + e)
  }
}

function closeDetailDialog() {
  showDetailDialog.value = false
  selectedSkill.value = null
}

function formatDate(dateString: string) {
  try {
    const date = new Date(dateString)
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return dateString
  }
}

onMounted(async () => {
  console.log('Skills component mounted')
  skillStore.loadSkills()
  
  // Listen for file drop events - try all possible event names
  try {
    console.log('Setting up file drop listeners...')
    
    // Try the standard Tauri 2.0 event name
    unlistenFileDrop = await listen<any>('tauri://drag-drop', (event) => {
      console.log('[tauri://drag-drop] File drop event received:', event)
      handleFileDropEvent(event.payload)
    })
    
    // Also try the old event name as fallback
    await listen<any>('tauri://file-drop', (event) => {
      console.log('[tauri://file-drop] File drop event received:', event)
      handleFileDropEvent(event.payload)
    })
    
    unlistenFileDropHover = await listen('tauri://drag-over', () => {
      console.log('[tauri://drag-over] File drop hover')
      if (showImportDialog.value && !importing.value && importType.value === 'zip') {
        isDragging.value = true
      }
    })
    
    await listen('tauri://file-drop-hover', () => {
      console.log('[tauri://file-drop-hover] File drop hover')
      if (showImportDialog.value && !importing.value && importType.value === 'zip') {
        isDragging.value = true
      }
    })
    
    unlistenFileDropCancelled = await listen('tauri://drag-leave', () => {
      console.log('[tauri://drag-leave] File drop cancelled')
      isDragging.value = false
    })
    
    await listen('tauri://file-drop-cancelled', () => {
      console.log('[tauri://file-drop-cancelled] File drop cancelled')
      isDragging.value = false
    })
    
    console.log('File drop listeners registered successfully')
  } catch (e) {
    console.error('Failed to setup file drop listeners:', e)
  }
})

function handleFileDropEvent(payload: any) {
  console.log('handleFileDropEvent called with:', payload)
  console.log('Dialog open:', showImportDialog.value, 'Type:', importType.value)
  
  if (!showImportDialog.value || importing.value || importType.value !== 'zip') {
    console.log('Ignoring file drop - dialog not open or wrong type')
    return
  }
  
  // Tauri 2.0 payload structure: { paths: string[], position: { x: number, y: number } }
  const paths = payload.paths || payload
  console.log('Extracted paths:', paths)
  
  if (paths && Array.isArray(paths) && paths.length > 0) {
    const filePath = paths[0]
    console.log('Processing file:', filePath)
    
    // Check if it's a ZIP file
    if (filePath.toLowerCase().endsWith('.zip')) {
      importSource.value = filePath
      selectedFileName.value = filePath.split(/[/\\]/).pop() || filePath
      errorMessage.value = ''
      isDragging.value = false
      console.log('File accepted:', selectedFileName.value)
    } else {
      errorMessage.value = '请选择 ZIP 文件'
      isDragging.value = false
      console.log('File rejected - not a ZIP file')
    }
  } else {
    console.log('Invalid paths format:', paths)
  }
}

onUnmounted(() => {
  // Clean up event listeners
  if (unlistenFileDrop) unlistenFileDrop()
  if (unlistenFileDropHover) unlistenFileDropHover()
  if (unlistenFileDropCancelled) unlistenFileDropCancelled()
})
</script>
