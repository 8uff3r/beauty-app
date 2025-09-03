<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'

// Mock data for shops
const shops = [
  {
    id: 1,
    name: 'Elite Barber Shop',
    rating: 4.8,
    distance: '0.5 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Beard Trim', 'Shave'],
    description: 'Premium barber shop with experienced stylists specializing in modern cuts and classic grooming.',
    owner: 'Michael Johnson',
    location: '123 Main St, Downtown',
    phone: '(555) 123-4567',
    availability: {
      monday: '9:00 AM - 7:00 PM',
      tuesday: '9:00 AM - 7:00 PM',
      wednesday: '9:00 AM - 7:00 PM',
      thursday: '9:00 AM - 7:00 PM',
      friday: '9:00 AM - 8:00 PM',
      saturday: '8:00 AM - 6:00 PM',
      sunday: '10:00 AM - 4:00 PM'
    }
  },
  {
    id: 2,
    name: 'Beauty Palace',
    rating: 4.9,
    distance: '1.2 km',
    image: 'https://placehold.co/300x200',
    services: ['Facial', 'Manicure', 'Pedicure'],
    description: 'Full-service beauty salon offering a wide range of treatments for men and women.',
    owner: 'Sarah Williams',
    location: '456 Oak Ave, Midtown',
    phone: '(555) 987-6543',
    availability: {
      monday: '10:00 AM - 6:00 PM',
      tuesday: '10:00 AM - 6:00 PM',
      wednesday: '10:00 AM - 6:00 PM',
      thursday: '10:00 AM - 6:00 PM',
      friday: '10:00 AM - 7:00 PM',
      saturday: '9:00 AM - 5:00 PM',
      sunday: 'Closed'
    }
  },
  {
    id: 3,
    name: 'Modern Cuts',
    rating: 4.7,
    distance: '0.8 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Coloring', 'Styling'],
    description: 'Trendy salon offering modern hairstyles and coloring techniques.',
    owner: 'David Chen',
    location: '789 Pine St, Uptown',
    phone: '(555) 456-7890',
    availability: {
      monday: '8:00 AM - 8:00 PM',
      tuesday: '8:00 AM - 8:00 PM',
      wednesday: '8:00 AM - 8:00 PM',
      thursday: '8:00 AM - 8:00 PM',
      friday: '8:00 AM - 9:00 PM',
      saturday: '7:00 AM - 6:00 PM',
      sunday: '9:00 AM - 3:00 PM'
    }
  }
]

const route = useRoute()
const router = useRouter()
const shop = ref<any>(null)

onMounted(() => {
  const shopId = parseInt(route.params.id as string)
  shop.value = shops.find(s => s.id === shopId) || shops[0]
})

const goBack = () => {
  router.back()
}
</script>

<template>
  <div class="p-4" v-if="shop">
    <!-- Header with back button -->
    <div class="flex items-center mb-4">
      <Button @click="goBack" variant="ghost" size="icon" class="mr-2">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-5 w-5">
          <path d="m15 18-6-6 6-6"/>
        </svg>
      </Button>
      <h1 class="text-2xl font-bold">{{ shop.name }}</h1>
    </div>

    <!-- Shop Image -->
    <img :src="shop.image" :alt="shop.name" class="w-full h-48 object-cover rounded-lg mb-4" />

    <!-- Rating and Distance -->
    <div class="flex items-center mb-4">
      <span class="mr-2">★ {{ shop.rating }}</span>
      <span>{{ shop.distance }}</span>
    </div>

    <!-- Description -->
    <Card class="mb-4">
      <CardHeader>
        <CardTitle>Description</CardTitle>
      </CardHeader>
      <CardContent>
        <p>{{ shop.description }}</p>
      </CardContent>
    </Card>

    <!-- Owner Info -->
    <Card class="mb-4">
      <CardHeader>
        <CardTitle>Owner</CardTitle>
      </CardHeader>
      <CardContent>
        <p>{{ shop.owner }}</p>
      </CardContent>
    </Card>

    <!-- Location and Contact -->
    <Card class="mb-4">
      <CardHeader>
        <CardTitle>Contact</CardTitle>
      </CardHeader>
      <CardContent>
        <p class="mb-2">📍 {{ shop.location }}</p>
        <p>📞 {{ shop.phone }}</p>
      </CardContent>
    </Card>

    <!-- Availability -->
    <Card class="mb-6">
      <CardHeader>
        <CardTitle>Availability</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-2 gap-2">
          <div v-for="(hours, day) in shop.availability" :key="day" class="flex justify-between">
            <span class="font-medium capitalize">{{ day }}:</span>
            <span>{{ hours }}</span>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Services -->
    <div class="mb-6">
      <h2 class="text-xl font-semibold mb-3">Services</h2>
      <div class="flex flex-wrap gap-2">
        <span 
          v-for="service in shop.services" 
          :key="service" 
          class="bg-muted px-3 py-1 rounded-full text-sm"
        >
          {{ service }}
        </span>
      </div>
    </div>

    <!-- Book Appointment Button -->
    <Button class="w-full">Book Appointment</Button>
  </div>
</template>