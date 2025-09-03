<script setup lang="ts">
import { ref } from 'vue'
import ShopCard from '@/components/ShopCard.vue'

// Mock data for featured shops
const featuredShops = ref([
  {
    id: 1,
    name: 'Elite Barber Shop',
    rating: 4.8,
    distance: '0.5 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Beard Trim', 'Shave']
  },
  {
    id: 2,
    name: 'Beauty Palace',
    rating: 4.9,
    distance: '1.2 km',
    image: 'https://placehold.co/300x200',
    services: ['Facial', 'Manicure', 'Pedicure']
  },
  {
    id: 3,
    name: 'Modern Cuts',
    rating: 4.7,
    distance: '0.8 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Coloring', 'Styling']
  }
])

const handleBook = (id: number) => {
  console.log(`Booking for shop ${id}`)
  // Implement booking logic here
}
</script>

<template>
  <div class="p-4">
    <h1 class="mb-4 text-2xl font-bold">Find Your Next Appointment</h1>
    
    <!-- Search bar -->
    <div class="mb-6">
      <div class="relative">
        <input 
          type="text" 
          placeholder="Search for barbers or beauty clinics..." 
          class="w-full p-3 rounded-lg border border-input bg-background"
        >
      </div>
    </div>
    
    <!-- Featured shops -->
    <div class="mb-6">
      <h2 class="text-xl font-semibold mb-3">Featured Shops</h2>
      <div class="space-y-4">
        <ShopCard 
          v-for="shop in featuredShops" 
          :key="shop.id" 
          :shop="shop" 
          @book="handleBook"
        />
      </div>
    </div>
  </div>
</template>