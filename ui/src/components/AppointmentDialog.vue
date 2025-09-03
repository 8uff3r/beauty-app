<script setup lang="ts">
import { ref, computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'

interface Appointment {
  id: number
  shopName: string
  service: string
  date: string
  time: string
  status: 'confirmed' | 'pending' | 'completed'
  location: string
  stylist: string
  price: string
  duration: string
}

interface Props {
  appointment: Appointment | null
  open: boolean
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const isOpen = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value)
})

const closeDialog = () => {
  isOpen.value = false
  emit('close')
}

const getStatusClass = (status: string) => {
  switch (status) {
    case 'confirmed':
      return 'bg-green-100 text-green-800'
    case 'pending':
      return 'bg-yellow-100 text-yellow-800'
    case 'completed':
      return 'bg-gray-100 text-gray-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
}
</script>

<template>
  <div v-if="isOpen && appointment" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
    <Card class="w-full max-w-md mx-4">
      <CardHeader class="p-4">
        <div class="flex justify-between items-center">
          <CardTitle class="text-lg">Appointment Details</CardTitle>
          <Button @click="closeDialog" variant="ghost" size="icon" class="h-6 w-6">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
              <path d="M18 6 6 18"/>
              <path d="m6 6 12 12"/>
            </svg>
          </Button>
        </div>
      </CardHeader>
      <CardContent class="p-4 pt-0">
        <div class="space-y-4">
          <div>
            <h3 class="font-semibold">{{ appointment.shopName }}</h3>
            <p class="text-sm text-muted-foreground">{{ appointment.service }}</p>
          </div>
          
          <div class="flex justify-between items-center">
            <div>
              <p class="font-medium">{{ appointment.date }} at {{ appointment.time }}</p>
              <span 
                class="text-xs px-2 py-1 rounded mt-1 inline-block"
                :class="getStatusClass(appointment.status)"
              >
                {{ appointment.status }}
              </span>
            </div>
          </div>
          
          <div class="grid grid-cols-2 gap-4">
            <div>
              <p class="text-sm text-muted-foreground">Location</p>
              <p>{{ appointment.location }}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Stylist</p>
              <p>{{ appointment.stylist }}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Duration</p>
              <p>{{ appointment.duration }}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Price</p>
              <p>{{ appointment.price }}</p>
            </div>
          </div>
          
          <div class="flex gap-2 pt-2">
            <Button variant="outline" class="flex-1">Reschedule</Button>
            <Button variant="destructive" class="flex-1">Cancel</Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>