import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Tool, DetectedTool } from '@/types'
import { invoke } from '@tauri-apps/api/core'

export const useToolStore = defineStore('tool', () => {
  const tools = ref<Tool[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const defaultTools: Tool[] = [
    {
      id: 'cursor',
      name: 'cursor',
      displayName: 'Cursor',
      skillsDir: '.cursor\\skills',
      detectDir: '.cursor',
      isInstalled: false,
    },
    {
      id: 'claude_code',
      name: 'claude_code',
      displayName: 'Claude Code',
      skillsDir: '.claude\\skills',
      detectDir: '.claude',
      isInstalled: false,
    },
    {
      id: 'codex',
      name: 'codex',
      displayName: 'Codex',
      skillsDir: '.codex\\skills',
      detectDir: '.codex',
      isInstalled: false,
    },
    {
      id: 'opencode',
      name: 'opencode',
      displayName: 'OpenCode',
      skillsDir: '.config\\opencode\\skill',
      detectDir: '.config\\opencode',
      isInstalled: false,
    },
    {
      id: 'antigravity',
      name: 'antigravity',
      displayName: 'Antigravity',
      skillsDir: '.gemini\\antigravity\\skills',
      detectDir: '.gemini\\antigravity',
      isInstalled: false,
    },
    {
      id: 'amp',
      name: 'amp',
      displayName: 'Amp',
      skillsDir: '.config\\agents\\skills',
      detectDir: '.config\\agents',
      isInstalled: false,
    },
    {
      id: 'kilo_code',
      name: 'kilo_code',
      displayName: 'Kilo Code',
      skillsDir: '.kilocode\\skills',
      detectDir: '.kilocode',
      isInstalled: false,
    },
    {
      id: 'roo_code',
      name: 'roo_code',
      displayName: 'Roo Code',
      skillsDir: '.roo\\skills',
      detectDir: '.roo',
      isInstalled: false,
    },
    {
      id: 'goose',
      name: 'goose',
      displayName: 'Goose',
      skillsDir: '.config\\goose\\skills',
      detectDir: '.config\\goose',
      isInstalled: false,
    },
    {
      id: 'gemini_cli',
      name: 'gemini_cli',
      displayName: 'Gemini CLI',
      skillsDir: '.gemini\\skills',
      detectDir: '.gemini',
      isInstalled: false,
    },
    {
      id: 'github_copilot',
      name: 'github_copilot',
      displayName: 'GitHub Copilot',
      skillsDir: '.copilot\\skills',
      detectDir: '.copilot',
      isInstalled: false,
    },
    {
      id: 'clawdbot',
      name: 'clawdbot',
      displayName: 'Clawdbot',
      skillsDir: '.clawdbot\\skills',
      detectDir: '.clawdbot',
      isInstalled: false,
    },
    {
      id: 'droid',
      name: 'droid',
      displayName: 'Droid',
      skillsDir: '.factory\\skills',
      detectDir: '.factory',
      isInstalled: false,
    },
    {
      id: 'windsurf',
      name: 'windsurf',
      displayName: 'Windsurf',
      skillsDir: '.codeium\\windsurf\\skills',
      detectDir: '.codeium\\windsurf',
      isInstalled: false,
    },
  ]

  // Load custom tools from localStorage
  function loadCustomTools(): Tool[] {
    try {
      const stored = localStorage.getItem('customTools')
      if (stored) {
        return JSON.parse(stored)
      }
    } catch (e) {
      console.error('Failed to load custom tools:', e)
    }
    return []
  }

  // Save custom tools to localStorage
  function saveCustomTools(customTools: Tool[]) {
    try {
      localStorage.setItem('customTools', JSON.stringify(customTools))
    } catch (e) {
      console.error('Failed to save custom tools:', e)
    }
  }

  async function detectTools() {
    loading.value = true
    error.value = null
    try {
      const detected = await invoke<DetectedTool[]>('detect_installed_tools')
      
      // Load custom tools
      const customTools = loadCustomTools()
      
      // Detect custom tools manually
      const customToolsWithDetection = await Promise.all(
        customTools.map(async (tool) => {
          try {
            // Check if detect directory exists
            const exists = await invoke<boolean>('check_directory_exists', { 
              path: tool.detectDir 
            })
            return {
              ...tool,
              isInstalled: exists,
              isCustom: true,
            }
          } catch (e) {
            console.error(`Failed to detect custom tool ${tool.id}:`, e)
            return {
              ...tool,
              isInstalled: false,
              isCustom: true,
            }
          }
        })
      )
      
      // Combine default tools and custom tools
      const defaultToolsWithDetection = defaultTools.map(tool => {
        const found = detected.find(d => d.id === tool.id)
        return {
          ...tool,
          isInstalled: found?.is_installed || false,
          installPath: found?.path,
          skillsDir: found?.skills_dir || tool.skillsDir,
        }
      })
      
      tools.value = [...defaultToolsWithDetection, ...customToolsWithDetection]
    } catch (e) {
      error.value = e as string
      console.error('Failed to detect tools:', e)
      
      // Fallback: show all tools as not installed
      const customTools = loadCustomTools().map(t => ({ ...t, isInstalled: false, isCustom: true }))
      tools.value = [...defaultTools, ...customTools]
    } finally {
      loading.value = false
    }
  }

  function addCustomTool(tool: Omit<Tool, 'isInstalled' | 'isCustom'>) {
    const customTools = loadCustomTools()
    const newTool: Tool = {
      ...tool,
      isInstalled: false,
      isCustom: true,
    }
    customTools.push(newTool)
    saveCustomTools(customTools)
  }

  function updateCustomTool(toolId: string, updates: Partial<Tool>) {
    const customTools = loadCustomTools()
    const index = customTools.findIndex(t => t.id === toolId)
    if (index !== -1) {
      customTools[index] = { ...customTools[index], ...updates }
      saveCustomTools(customTools)
    }
  }

  function deleteCustomTool(toolId: string) {
    const customTools = loadCustomTools()
    const filtered = customTools.filter(t => t.id !== toolId)
    saveCustomTools(filtered)
  }

  function getInstalledTools() {
    return tools.value.filter(t => t.isInstalled)
  }

  return {
    tools,
    loading,
    error,
    detectTools,
    getInstalledTools,
    addCustomTool,
    updateCustomTool,
    deleteCustomTool,
  }
})
