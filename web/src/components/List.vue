<script setup lang="ts">
import { useTodoStore } from "../stores/todos";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";

const store = useTodoStore();

const { todos } = storeToRefs(store);

onMounted(() => {
  store.todo_list();
});
</script>

<template>
  <div v-for="(item, i) in todos" :key="i">{{ item.description }}</div>
</template>
