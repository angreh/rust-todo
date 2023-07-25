<script setup lang="ts">
import DefaultInput from "@/components/Input.vue";
import TodoList from "@/components/List.vue";
import TodoTitle from "@/components/Title.vue";
</script>

<template>
  <div id="main_div">
    <TodoTitle />
    <DefaultInput />
    <TodoList />
  </div>
</template>
