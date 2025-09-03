<template>
  <div class="toc-panel">
    <div class="toc-header">
      <div class="header-title">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
          <path d="M3 6h18M3 12h18M3 18h18" stroke="currentColor" stroke-width="2"/>
        </svg>
        <span>Table of Contents</span>
      </div>
    </div>
    
    <div class="toc-content">
      <div 
        v-for="heading in headings" 
        :key="heading.id"
        class="toc-item"
        :class="`level-${heading.level}`"
        @click="scrollToHeading(heading.id)"
      >
        <span class="toc-text">{{ heading.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Heading {
  id: string
  text: string
  level: number
}

const props = defineProps<{
  content: string
}>()

const headings = computed(() => {
  const headingRegex = /^(#{1,6})\s+(.+)$/gm
  const matches: Heading[] = []
  let match
  
  while ((match = headingRegex.exec(props.content)) !== null) {
    const level = match[1].length
    const text = match[2].trim()
    const id = text.toLowerCase().replace(/[^\w\s-]/g, '').replace(/\s+/g, '-')
    
    matches.push({
      id,
      text,
      level
    })
  }
  
  return matches
})

const scrollToHeading = (id: string) => {
  // 这里可以实现滚动到对应标题的功能
  console.log('Scroll to:', id)
}
</script>

<style scoped>
.toc-panel {
  width: 250px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  display: flex;
  flex-direction: column;
  border-left: 1px solid var(--border-color);
  transition: background-color 0.3s ease, color 0.3s ease;
}

.toc-header {
  height: 35px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  border-bottom: 1px solid var(--border-color);
  font-size: 11px;
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-primary);
}

.toc-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.toc-item {
  padding: 3px 12px;
  cursor: pointer;
  font-size: 13px;
  line-height: 1.4;
  transition: background-color 0.2s;
  border-left: 2px solid transparent;
}

.toc-item:hover {
  background: var(--hover-bg);
  border-left-color: var(--accent-color);
}

.toc-item.level-1 {
  padding-left: 12px;
  font-weight: 600;
}

.toc-item.level-2 {
  padding-left: 24px;
}

.toc-item.level-3 {
  padding-left: 36px;
  font-size: 12px;
}

.toc-item.level-4 {
  padding-left: 48px;
  font-size: 12px;
}

.toc-item.level-5 {
  padding-left: 60px;
  font-size: 11px;
}

.toc-item.level-6 {
  padding-left: 72px;
  font-size: 11px;
}

.toc-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: block;
}
</style>