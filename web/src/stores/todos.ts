import { ref } from "vue";
import { defineStore, acceptHMRUpdate } from "pinia";
import { list, create, remove, get, update } from "../services/todo";

export const useTodoStore = defineStore("todo", () => {
  const todos = ref<Array<any>>([]);

  async function todo_list() {
    todos.value = [];

    const result = await list();

    todos.value = result.data.list;
  }

  async function todo_remove(id: string) {
    await remove(id);

    // TODO: not do this if has a error
    const index = todos.value.findIndex((item) => item._id.$oid == id);
    todos.value.splice(index, 1);
  }

  async function todo_create(description: string) {
    const result = await create(description);

    // TODO: not do this if has a error
    todos.value.push({
      description,
      _id: {
        $oid: result.data.id.$oid,
      },
    });
  }

  async function todo_get(id: string) {
    return await get(id);
  }

  async function todo_update(item: any) {
    await update(item);
    todo_list();
  }

  return {
    todo_create,
    todo_get,
    todo_list,
    todo_remove,
    todo_update,
    todos,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useTodoStore, import.meta.hot));
}
