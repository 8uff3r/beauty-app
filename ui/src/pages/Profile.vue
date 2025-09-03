<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { useDarkMode } from '@/composables/useDarkMode'

const { isDarkMode, toggleDarkMode } = useDarkMode()

const user = ref({
  name: 'John Doe',
  email: 'john.doe@example.com',
  phone: '+1 (555) 123-4567',
  memberSince: 'Jan 2023'
})

const menuItems = ref([
  { id: 1, title: 'Personal Information', icon: 'User' },
  { id: 2, title: 'Payment Methods', icon: 'CreditCard' },
  { id: 3, title: 'Notification Settings', icon: 'Bell' },
  { id: 4, title: 'Privacy Settings', icon: 'Lock' },
  { id: 5, title: 'Help & Support', icon: 'HelpCircle' }
])
</script>

<template>
  <div class="p-4">
    <h1 class="mb-4 text-2xl font-bold">Profile</h1>
    
    <!-- User info card -->
    <Card class="mb-6">
      <CardContent class="p-4">
        <div class="flex items-center">
          <div class="bg-gray-200 border-2 border-dashed rounded-xl w-16 h-16" />
          <div class="ml-4">
            <h2 class="text-xl font-semibold">{{ user.name }}</h2>
            <p class="text-muted-foreground">{{ user.email }}</p>
            <p class="text-muted-foreground">{{ user.phone }}</p>
          </div>
        </div>
        <div class="mt-4 pt-4 border-t border-border">
          <p class="text-sm text-muted-foreground">Member since {{ user.memberSince }}</p>
        </div>
      </CardContent>
    </Card>
    
    <!-- Dark mode toggle -->
    <Card class="mb-6">
      <CardHeader class="p-4">
        <CardTitle class="text-lg">Appearance</CardTitle>
      </CardHeader>
      <CardContent class="p-4 pt-0">
        <div class="flex items-center justify-between">
          <Label for="dark-mode" class="font-medium">Dark Mode</Label>
          <Switch 
            id="dark-mode" 
            :checked="isDarkMode" 
            @update:checked="toggleDarkMode"
          />
        </div>
      </CardContent>
    </Card>
    
    <!-- Menu items -->
    <div class="space-y-3">
      <Card v-for="item in menuItems" :key="item.id" class="cursor-pointer hover:bg-muted">
        <CardContent class="p-4 flex items-center">
          <div class="bg-gray-200 border-2 border-dashed rounded-xl w-8 h-8 mr-3" />
          <span class="font-medium">{{ item.title }}</span>
        </CardContent>
      </Card>
    </div>
    
    <!-- Logout button -->
    <div class="mt-6">
      <Button variant="destructive" class="w-full">Logout</Button>
    </div>
  </div>
</template>