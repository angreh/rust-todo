<script setup lang="ts">
import { useTodoStore } from "@/stores/todos";
import useEventBus from "@/composables/useEventBus";
import { storeToRefs } from "pinia";
import { onMounted, onBeforeUnmount } from "vue";

const store = useTodoStore();

const { todos } = storeToRefs(store);

onMounted(() => {
  store.todo_list();
});

function remove(id: string) {
  store.todo_remove(id);
}

// event bus
const { on, off } = useEventBus();
function show_value(value: string) {
  console.log(value);
}

onMounted(()=>{
  on("todo_created", show_value);
  on("todo_created2", show_value);
})

onBeforeUnmount(()=>{
  off("todo_created", show_value)
  off("todo_created2", show_value);
})
</script>

<template>
  <TransitionGroup name="list">
    <div v-for="item in todos" :key="item._id.$oid">
      {{ item.description }} (<span @click="remove(item._id.$oid)">close</span>)
      (<span @click="() => {}">edit</span>)
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
