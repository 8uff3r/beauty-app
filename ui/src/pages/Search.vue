<script setup lang="ts">
import { ref } from 'vue'
import ShopCard from '@/components/ShopCard.vue'

const searchQuery = ref('')
const searchResults = ref([
  {
    id: 1,
    name: 'Premium Barber Lounge',
    rating: 4.9,
    distance: '0.3 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Beard Trim', 'Shave', 'Hot Towel']
  },
  {
    id: 2,
    name: 'Glamour Beauty Studio',
    rating: 4.8,
    distance: '1.5 km',
    image: 'https://placehold.co/300x200',
    services: ['Facial', 'Waxing', 'Makeup', 'Massage']
  },
  {
    id: 3,
    name: 'Urban Cuts',
    rating: 4.6,
    distance: '0.9 km',
    image: 'https://placehold.co/300x200',
    services: ['Haircut', 'Coloring', 'Styling', 'Treatment']
  }
])

const handleBook = (id: number) => {
  console.log(`Booking for shop ${id}`)
  // Implement booking logic here
}
</script>

<template>
  <div class="p-4">
    <h1 class="mb-4 text-2xl font-bold">Search</h1>
    
    <!-- Search bar -->
    <div class="mb-6">
      <div class="relative">
        <input 
          v-model="searchQuery"
          type="text" 
          placeholder="Search for barbers or beauty clinics..." 
          class="w-full p-3 rounded-lg border border-input bg-background"
        >
      </div>
    </div>
    
    <!-- Search results -->
    <div>
      <h2 class="text-xl font-semibold mb-3">Results</h2>
      <div class="space-y-4">
        <ShopCard 
          v-for="shop in searchResults" 
          :key="shop.id" 
          :shop="shop" 
          @book="handleBook"
        />
      </div>
    </div>
  </div>
</template>