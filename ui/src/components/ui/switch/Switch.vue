<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  checked?: boolean
  disabled?: boolean
}

interface Emits {
  (e: 'update:checked', value: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
  checked: false,
  disabled: false
})

const emit = defineEmits<Emits>()

const model = computed({
  get: () => props.checked,
  set: (value) => emit('update:checked', value)
})

const toggle = () => {
  if (!props.disabled) {
    model.value = !model.value
  }
}
</script>

<template>
  <button
    role="switch"
    :aria-checked="model"
    :disabled="disabled"
    @click="toggle"
    class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
    :class="{
      'bg-primary': model,
      'bg-input': !model,
      'opacity-50 cursor-not-allowed': disabled
    }"
  >
    <span
      class="inline-block h-5 w-5 transform rounded-full bg-background transition-transform"
      :class="{
        'translate-x-6': model,
        'translate-x-1': !model
      }"
    />
  </button>
</template>