<script setup lang="ts">
import { useTodoStore } from "../stores/todos";

const store = useTodoStore();

function save() {
  // store.todo_remove(id);
}
</script>

<template><input type="text" /><button @click="save">save</button></template>
