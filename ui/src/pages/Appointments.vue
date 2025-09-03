<script setup lang="ts">
import { ref } from 'vue'
import AppointmentCard from '@/components/AppointmentCard.vue'
import AppointmentDialog from '@/components/AppointmentDialog.vue'

const appointments = ref([
  {
    id: 1,
    shopName: 'Elite Barber Shop',
    service: 'Haircut & Beard Trim',
    date: 'Today',
    time: '3:00 PM',
    status: 'confirmed',
    location: '123 Main St, Downtown',
    stylist: 'Michael Johnson',
    price: '$35',
    duration: '45 mins'
  },
  {
    id: 2,
    shopName: 'Beauty Palace',
    service: 'Facial & Manicure',
    date: 'Tomorrow',
    time: '10:30 AM',
    status: 'pending',
    location: '456 Oak Ave, Midtown',
    stylist: 'Sarah Williams',
    price: '$85',
    duration: '1.5 hours'
  },
  {
    id: 3,
    shopName: 'Modern Cuts',
    service: 'Hair Coloring',
    date: 'Jun 15',
    time: '2:00 PM',
    status: 'completed',
    location: '789 Pine St, Uptown',
    stylist: 'David Chen',
    price: '$120',
    duration: '2 hours'
  }
])

const selectedAppointment = ref<any>(null)
const isDialogOpen = ref(false)

const handleView = (id: number) => {
  selectedAppointment.value = appointments.value.find(a => a.id === id) || null
  isDialogOpen.value = true
}
</script>

<template>
  <div class="p-4">
    <h1 class="mb-4 text-2xl font-bold">My Appointments</h1>
    
    <!-- Upcoming appointments -->
    <div class="mb-6">
      <h2 class="text-xl font-semibold mb-3">Upcoming</h2>
      <div class="space-y-4">
        <AppointmentCard 
          v-for="appointment in appointments" 
          :key="appointment.id" 
          :appointment="appointment" 
          @view="handleView"
        />
      </div>
    </div>
    
    <!-- Past appointments -->
    <div>
      <h2 class="text-xl font-semibold mb-3">Past Appointments</h2>
      <p class="text-muted-foreground">Your past appointments will appear here.</p>
    </div>
    
    <!-- Appointment Dialog -->
    <AppointmentDialog 
      :appointment="selectedAppointment" 
      :open="isDialogOpen" 
      @update:open="isDialogOpen = $event"
      @close="selectedAppointment = null"
    />
  </div>
</template>