<script setup lang="ts">
import { storeToRefs } from "pinia";
import { onMounted, onBeforeUnmount } from "vue";

import { useTodoStore } from "@/stores/todos";
import useEventBus from "@/composables/useEventBus";

const store = useTodoStore();

const { todos } = storeToRefs(store);

onMounted(() => {
  store.todo_list();
});

function remove(id: string) {
  store.todo_remove(id);
}

function selectItem(id: string) {
  store.todo_select(id);
}

// event bus
const { on, off } = useEventBus();
function show_value(value: string) {
  console.log(value);
}

onMounted(() => {
  on("todo_created", show_value);
  on("todo_created2", show_value);
});

onBeforeUnmount(() => {
  off("todo_created", show_value);
  off("todo_created2", show_value);
});
</script>

<template>
  <TransitionGroup name="list">
    <div v-for="item in todos" :key="item.id">
      {{ item.description }} (<span @click="remove(item.id)">close</span>)
      (<span @click="selectItem(item.id)">edit</span>)
    </div>
  </TransitionGroup>
</template>

<style>
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-50px);
}
.list-leave-active {
  position: absolute;
}
</style>
