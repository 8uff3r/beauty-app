import { ref, onMounted } from 'vue'

const isDarkMode = ref(false)

export function useDarkMode() {
  const toggleDarkMode = () => {
    isDarkMode.value = !isDarkMode.value
    document.documentElement.classList.toggle('dark', isDarkMode.value)
    localStorage.setItem('darkMode', isDarkMode.value.toString())
  }

  const setDarkMode = (value: boolean) => {
    isDarkMode.value = value
    document.documentElement.classList.toggle('dark', value)
    localStorage.setItem('darkMode', value.toString())
  }

  onMounted(() => {
    const savedDarkMode = localStorage.getItem('darkMode')
    if (savedDarkMode !== null) {
      setDarkMode(savedDarkMode === 'true')
    } else {
      // Check system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      setDarkMode(prefersDark)
    }
  })

  return {
    isDarkMode,
    toggleDarkMode,
    setDarkMode
  }
}