import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Skill, SkillDetail } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { useToolStore } from './tool'

export const useSkillStore = defineStore('skill', () => {
  const skills = ref<Skill[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadSkills() {
    loading.value = true
    error.value = null
    try {
      skills.value = await invoke<Skill[]>('get_skills')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load skills:', e)
    } finally {
      loading.value = false
    }
  }

  async function getSkillDetail(skillId: string): Promise<SkillDetail | null> {
    try {
      return await invoke<SkillDetail>('get_skill_detail', { skillId })
    } catch (e) {
      error.value = e as string
      console.error('Failed to get skill detail:', e)
      return null
    }
  }

  async function importSkill(source: string, sourceType: 'local' | 'git' | 'zip') {
    loading.value = true
    error.value = null
    try {
      await invoke('import_skill', { source, sourceType })
      await loadSkills()
    } catch (e) {
      error.value = e as string
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteSkill(skillId: string) {
    loading.value = true
    error.value = null
    try {
      await invoke('delete_skill', { skillId })
      await loadSkills()
    } catch (e) {
      error.value = e as string
      throw e
    } finally {
      loading.value = false
    }
  }

  async function syncSkill(skillId: string, toolId: string, useSymlink: boolean = true) {
    try {
      // Check if this is a custom tool by looking it up in the tool store
      const toolStore = useToolStore()
      const tool = toolStore.tools.find(t => t.id === toolId)
      
      if (tool?.isCustom) {
        // Use custom path for custom tools
        return await invoke('sync_skill_with_path', { 
          skillId, 
          toolId, 
          skillsDir: tool.skillsDir,
          useSymlink 
        })
      } else {
        // Use built-in detection for default tools
        return await invoke('sync_skill', { skillId, toolId, useSymlink })
      }
    } catch (e) {
      error.value = e as string
      throw e
    }
  }

  return {
    skills,
    loading,
    error,
    loadSkills,
    getSkillDetail,
    importSkill,
    deleteSkill,
    syncSkill,
  }
})
