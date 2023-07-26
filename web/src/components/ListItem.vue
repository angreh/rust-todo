<script lang="ts" setup>
import { useTodoStore } from "@/stores/todos";

const store = useTodoStore();

defineProps(["item"]);

function remove(id: string) {
  store.todo_remove(id);
}

function edit(id: string) {
  store.todo_select(id);
}
</script>

<template>
  <div class="list-item">
    <div class="list-item--title">
      {{ item.description }}
    </div>

    <div class="list-item--action">
      <div @click="remove(item.id)" class="item-button remove">
        <img src="/close.svg" class="icon" />
      </div>
      <div @click="edit(item.id)" class="item-button edit">
        <img src="/edit.svg" class="icon" />
      </div>
    </div>
  </div>
</template>
