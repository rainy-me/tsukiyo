<template>
  <div class="lg:mx-10 lg:my-0 my-10">
    <img src="/assets/DSC08500.jpeg" />
    <div class="text-white mt-6">{{ title }}</div>
    <div class="divider"></div>
    <div class="text-white mt-6">{{ text }}</div>
  </div>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  imgPath?: string;
  title: string;
  text: string;
}>();
</script>

<style scoped>
.divider {
  display: block;
  width: 80px;
  height: 4px;
  background-color: #7a6459;
  margin: 0 auto;
}
</style>