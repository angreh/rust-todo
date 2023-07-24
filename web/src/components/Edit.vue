<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useTodoStore } from "../stores/todos";

const store = useTodoStore();

const { selecteditem } = storeToRefs(store);

function save() {
  store.todo_update();
}
</script>

<template>
  <br />
  <input type="text" v-model="selecteditem.description" />
  <button @click="save" :disabled="!selecteditem.id">save</button>
</template>
