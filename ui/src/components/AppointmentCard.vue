<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

interface Appointment {
  id: number
  shopName: string
  service: string
  date: string
  time: string
  status: 'confirmed' | 'pending' | 'completed'
}

interface Props {
  appointment: Appointment
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'view', id: number): void
}>()

const handleView = () => {
  emit('view', props.appointment.id)
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
  <Card class="overflow-hidden">
    <CardHeader class="p-3">
      <CardTitle class="text-lg">{{ appointment.shopName }}</CardTitle>
      <CardDescription>{{ appointment.service }}</CardDescription>
    </CardHeader>
    <CardContent class="p-3 pt-0">
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
        <Button @click="handleView" size="sm" variant="outline">View</Button>
      </div>
    </CardContent>
  </Card>
</template>