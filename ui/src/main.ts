import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import './index.css';

// Initialize dark mode
const initDarkMode = () => {
  const savedDarkMode = localStorage.getItem('darkMode');
  if (savedDarkMode !== null) {
    document.documentElement.classList.toggle('dark', savedDarkMode === 'true');
  } else {
    // Check system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.classList.toggle('dark', prefersDark);
  }
};

initDarkMode();

const app = createApp(App);
app.use(router);
app.mount('#app');
