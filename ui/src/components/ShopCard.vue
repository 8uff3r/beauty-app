<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { useRouter } from 'vue-router'

interface Service {
  id: number
  name: string
}

interface Shop {
  id: number
  name: string
  rating: number
  distance: string
  image: string
  services: string[]
}

interface Props {
  shop: Shop
}

const props = defineProps<Props>()
const router = useRouter()

const emit = defineEmits<{
  (e: 'book', id: number): void
  (e: 'view-details', id: number): void
}>()

const handleBook = () => {
  emit('book', props.shop.id)
}

const handleViewDetails = () => {
  emit('view-details', props.shop.id)
  router.push(`/shop/${props.shop.id}`)
}
</script>

<template>
  <Card class="overflow-hidden cursor-pointer" @click="handleViewDetails">
    <img :src="shop.image" :alt="shop.name" class="w-full h-32 object-cover" />
    <CardHeader class="p-3">
      <CardTitle class="text-lg">{{ shop.name }}</CardTitle>
      <CardDescription class="flex items-center">
        <span class="mr-2">★ {{ shop.rating }}</span>
        <span>{{ shop.distance }}</span>
      </CardDescription>
    </CardHeader>
    <CardContent class="p-3 pt-0">
      <div class="flex flex-wrap gap-1 mb-3">
        <span 
          v-for="service in shop.services" 
          :key="service" 
          class="text-xs bg-muted px-2 py-1 rounded"
        >
          {{ service }}
        </span>
      </div>
      <Button @click.stop="handleBook" size="sm" class="w-full">Book Now</Button>
    </CardContent>
  </Card>
</template>