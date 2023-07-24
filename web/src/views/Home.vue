<script setup lang="ts">
import DefaultInput from "@/components/Input.vue";
import TodoList from "@/components/List.vue";
import EditInput from "@/components/Edit.vue";
</script>

<template>
  <div id="main_div">
    Title todo thing:
    <DefaultInput />
    <TodoList />
    <EditInput />
  </div>
</template>
