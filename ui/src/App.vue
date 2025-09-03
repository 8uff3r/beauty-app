<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Home, Search, Calendar, User } from 'lucide-vue-next'
import MainLayout from './components/layout/MainLayout.vue'
import { useDarkMode } from '@/composables/useDarkMode'

const route = useRoute()
const router = useRouter()
const { isDarkMode } = useDarkMode()

// Navigation items
const navItems = [
  { name: 'Home', icon: Home, path: '/' },
  { name: 'Search', icon: Search, path: '/search' },
  { name: 'Appointments', icon: Calendar, path: '/appointments' },
  { name: 'Profile', icon: User, path: '/profile' }
]

// Current active tab
const activeTab = ref('/')

// Watch for route changes
watch(route, (newRoute) => {
  activeTab.value = newRoute.path
})

// Navigate to a tab
const navigateTo = (path: string) => {
  activeTab.value = path
  router.push(path)
}
</script>

<template>
  <div 
    id="app" 
    class="flex min-h-screen flex-col select-none"
    :class="{ 'dark': isDarkMode }"
  >
    <MainLayout>
      <router-view />
    </MainLayout>
    
    <!-- Bottom Navigation -->
    <nav class="fixed bottom-0 left-0 right-0 bg-background border-t border-border">
      <div class="flex justify-around items-center py-2">
        <button 
          v-for="item in navItems"
          :key="item.path"
          @click="navigateTo(item.path)"
          class="flex flex-col items-center justify-center p-2 rounded-lg w-1/4"
          :class="{ 'text-primary': activeTab === item.path, 'text-muted-foreground': activeTab !== item.path }"
        >
          <component :is="item.icon" class="h-5 w-5" />
          <span class="text-xs mt-1">{{ item.name }}</span>
        </button>
      </div>
    </nav>
  </div>
</template>

<style scoped>
/* Add any specific styles for App.vue here if needed */
</style>

