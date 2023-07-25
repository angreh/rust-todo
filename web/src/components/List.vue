<script setup lang="ts">
import { storeToRefs } from "pinia";
import { onMounted } from "vue";
// import { onMounted, onBeforeUnmount } from "vue";

import { useTodoStore } from "@/stores/todos";
// import useEventBus from "@/composables/useEventBus";

import ListItem from "./ListItem.vue";

const store = useTodoStore();

const { todos } = storeToRefs(store);
onMounted(() => {
  store.todo_list();
});


// function selectItem(id: string) {
//   store.todo_select(id);
// }

// event bus
// const { on, off } = useEventBus();
// function show_value(value: string) {
//   console.log(value);
// }

// onMounted(() => {
//   on("todo_created", show_value);
//   on("todo_created2", show_value);
// });

// onBeforeUnmount(() => {
//   off("todo_created", show_value);
//   off("todo_created2", show_value);
// });
</script>

<template>
  <TransitionGroup name="list">
    <div v-for="item in todos" :key="item.id">
      <ListItem :item="item" />
    </div>
  </TransitionGroup>
  <br />
  <br />
  <br />
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
