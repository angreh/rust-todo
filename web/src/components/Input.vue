<script setup lang="ts">
import { ref } from "vue";
import { useTodoStore } from "../stores/todos";

const store = useTodoStore();
const description = ref<string>("");
function create() {
  store.todo_create(description.value);
}
</script>

<template>
  <div>
    <input type="text" v-model="description" /><button @click="create">
      yay
    </button>
  </div>
</template>
