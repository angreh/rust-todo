import { ref } from "vue";
import { defineStore, acceptHMRUpdate } from "pinia";
import { list, create } from "../services/todo";

export const useTodoStore = defineStore("todo", () => {
  const todos = ref<Array<any>>([]);

  async function todo_list() {
    const result = await list();

    const list_todos: any[] = result.data.list;

    const len = list_todos.length;
    let item;
    for (let i = 0; i < len; i++) {
      item = list_todos[i];
      todos.value.push(item);
    }
  }

  async function todo_create(description: string) {
    await create(description);
    todo_list();
  }

  return {
    todo_create,
    todo_list,
    todos,
  };
});

// make sure to pass the right store definition, `useAuth` in this case.
if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useTodoStore, import.meta.hot));
}
