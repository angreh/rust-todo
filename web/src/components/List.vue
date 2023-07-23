<script setup lang="ts">
import { useTodoStore } from "../stores/todos";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";

const store = useTodoStore();

const { todos } = storeToRefs(store);

onMounted(() => {
  store.todo_list();
});

function remove(id: string) {
  store.todo_remove(id);
}

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
