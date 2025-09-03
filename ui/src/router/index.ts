import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/Home.vue'
import Search from '@/pages/Search.vue'
import Appointments from '@/pages/Appointments.vue'
import Profile from '@/pages/Profile.vue'
import ShopDetails from '@/pages/ShopDetails.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/search', component: Search },
  { path: '/appointments', component: Appointments },
  { path: '/profile', component: Profile },
  { path: '/shop/:id', component: ShopDetails, props: true }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router