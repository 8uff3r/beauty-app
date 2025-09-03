<script setup lang="ts">
import { LabelHTMLAttributes } from 'vue';

interface Props extends /* @vue-ignore */ LabelHTMLAttributes {
  for?: string;
}

const props = defineProps<Props>();
</script>

<template>
  <label
    :for="props.for"
    class="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
  >
    <slot />
  </label>
</template>

