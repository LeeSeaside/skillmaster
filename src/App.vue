<template>
  <div class="app-container">
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useToolStore } from '@/stores/tool'

const toolStore = useToolStore()

onMounted(async () => {
  await toolStore.detectTools()
})
</script>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}
</style>
