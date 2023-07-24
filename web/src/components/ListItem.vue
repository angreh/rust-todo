<script lang="ts" setup>
import { useTodoStore } from "@/stores/todos";

const store = useTodoStore();

const props = defineProps(["item"]);

function remove(id: string) {
  store.todo_remove(id);
}
</script>

<template>
  <div class="list-item">
    <div class="list-item--title">
      {{ item.description }}
    </div>

    <div class="list-item--action">
      (&nbsp;<div @click="remove(item.id)">delete</div>&nbsp;|&nbsp;<div>edit</div>&nbsp;)
    </div>
  </div>
</template>
