<script setup lang="ts">
import { useTodoStore } from "../stores/todos";
import { storeToRefs } from "pinia";
import { onMounted, reactive } from "vue";

const store = useTodoStore();

const { todos } = storeToRefs(store);
let item = reactive({
  id: "",
  description: "",
});

onMounted(() => {
  store.todo_list();
});

async function edit(id: string) {
  const result = await store.todo_get(id);
  const todo = result.data.todo;

  item.description = todo.description;
  item.id = todo._id.$oid;
}

async function save() {
  await store.todo_update(item);
  item.description = "";
  item.id = "";
}

function remove(id: string) {
  store.todo_remove(id);
}
</script>

<template>
  <div v-for="(item, i) in todos" :key="i">
    {{ item.description }} (<span @click="remove(item._id.$oid)">close</span>)
    (<span @click="edit(item._id.$oid)">edit</span>)
  </div>
  <input type="text" v-model="item.description" /><button
    @click="save"
    :disabled="!item.description && !item.id">
    save
  </button>
</template>
