import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export type ThemeMode = 'light' | 'dark' | 'auto'

export const useThemeStore = defineStore('theme', () => {
  // 状态
  const mode = ref<ThemeMode>('auto')
  const systemPrefersDark = ref(false)

  // 计算属性
  const isDark = computed(() => {
    if (mode.value === 'auto') {
      return systemPrefersDark.value
    }
    return mode.value === 'dark'
  })

  const currentTheme = computed(() => {
    return isDark.value ? 'dark' : 'light'
  })

  // 方法
  const setTheme = (newMode: ThemeMode) => {
    mode.value = newMode
    localStorage.setItem('theme-mode', newMode)
    applyTheme()
  }

  const toggleTheme = () => {
    if (mode.value === 'light') {
      setTheme('dark')
    } else if (mode.value === 'dark') {
      setTheme('auto')
    } else {
      setTheme('light')
    }
  }

  const applyTheme = () => {
    const root = document.documentElement
    
    if (isDark.value) {
      root.classList.add('dark')
      root.classList.remove('light')
    } else {
      root.classList.add('light')
      root.classList.remove('dark')
    }
    
    // 设置CSS变量
    if (isDark.value) {
      root.style.setProperty('--bg-primary', '#1e1e1e')
      root.style.setProperty('--bg-secondary', '#252526')
      root.style.setProperty('--bg-tertiary', '#2d2d30')
      root.style.setProperty('--text-primary', '#cccccc')
      root.style.setProperty('--text-secondary', '#969696')
      root.style.setProperty('--border-color', '#3e3e42')
      root.style.setProperty('--accent-color', '#007acc')
      root.style.setProperty('--hover-bg', '#2a2d2e')
    } else {
      root.style.setProperty('--bg-primary', '#ffffff')
      root.style.setProperty('--bg-secondary', '#f8f9fa')
      root.style.setProperty('--bg-tertiary', '#e9ecef')
      root.style.setProperty('--text-primary', '#212529')
      root.style.setProperty('--text-secondary', '#6c757d')
      root.style.setProperty('--border-color', '#dee2e6')
      root.style.setProperty('--accent-color', '#0d6efd')
      root.style.setProperty('--hover-bg', '#f8f9fa')
    }
  }

  const initTheme = () => {
    // 检测系统主题偏好
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    systemPrefersDark.value = mediaQuery.matches
    
    // 监听系统主题变化
    mediaQuery.addEventListener('change', (e) => {
      systemPrefersDark.value = e.matches
      if (mode.value === 'auto') {
        applyTheme()
      }
    })

    // 从localStorage恢复主题设置
    const savedMode = localStorage.getItem('theme-mode') as ThemeMode
    if (savedMode && ['light', 'dark', 'auto'].includes(savedMode)) {
      mode.value = savedMode
    }

    applyTheme()
  }

  return {
    mode,
    isDark,
    currentTheme,
    setTheme,
    toggleTheme,
    initTheme
  }
})