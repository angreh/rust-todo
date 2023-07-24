<script setup lang="ts">
import { ref } from "vue";
import { useTodoStore } from "@/stores/todos";
import useEventBus from "@/composables/useEventBus";
// const { emit } = useEventBus();

const store = useTodoStore();
const description = ref<string>("");
function create() {
  // emit("todo_created", description.value);
  // emit("todo_created2");
  store.todo_create(description.value);
  description.value = "";
}
</script>

<template>
  <div>
    <input type="text" v-model="description" @keyup.enter="create" /><button
      @click="create"
      :disabled="!description">
      yay
    </button>
  </div>
</template>
