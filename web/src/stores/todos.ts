import { ref, reactive } from "vue";
import { defineStore, acceptHMRUpdate } from "pinia";
import { list, create, remove, get, update } from "../services/todo";

type ListItem = {
  id: string;
  controlID: string;
  description: string;
};
type ListItemPartial = { id: string; description: string };

export const useTodoStore = defineStore("todo", () => {
  const todos = ref<ListItem[]>([]);
  const selecteditem = reactive<ListItemPartial>({
    id: "",
    description: "",
  });

  async function todo_list() {
    todos.value = [];

    const result = await list();

    todos.value = result.data.list.map((item: any) => ({
      id: item._id.$oid,
      controlID: item._id.$oid,
      description: item.description,
    }));
  }

  async function todo_remove(id: string) {
    let cachedItem = todos.value.find((item) => item.id == id)!;

    // front remove
    const index = todos.value.findIndex((item) => item.id == id);
    todos.value.splice(index, 1);

    try {
      // api remove
      await remove(id);
    } catch (_) {
      // rollback front
      setTimeout(() => {
        todos.value.splice(index, 0, cachedItem);
      }, 1000);
    }
  }

  async function todo_create(description: string) {
    const controlID = Date.now().toString();

    // front create
    todos.value.push({
      id: "",
      controlID,
      description,
    });

    try {
      // api create
      const result = await create(description);

      // put real id
      const item = todos.value.find((item) => item.controlID == controlID)!;
      item.id = result.data.id.$oid;
    } catch (_) {
      // rollback front
      setTimeout(() => {
        const index = todos.value.findIndex(
          (item) => item.controlID == controlID
        );
        todos.value.splice(index, 1);
      }, 1000);
    }
  }

  async function todo_get(id: string) {
    return await get(id);
  }

  async function todo_select(id: string) {
    const listItem = todos.value.find((item) => item.id == id)!;

    selecteditem.id = listItem.id;
    selecteditem.description = listItem.description;
  }

  async function todo_update() {
    const listItem = todos.value.find((item) => item.id == selecteditem.id)!;
    const cachedDescription = listItem.description;

    // front edit
    listItem.description = selecteditem.description;

    // clear
    selecteditem.id = "";
    selecteditem.description = "";

    try {
      // api edit
      await update({
        id: listItem.id,
        description: listItem.description,
      });
    } catch (_) {
      // rollback front
      setTimeout(() => {
        listItem.description = cachedDescription;
      }, 1000);
    }
  }

  return {
    todo_create,
    todo_get,
    todo_list,
    todo_remove,
    todo_select,
    todo_update,
    todos,
    selecteditem,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useTodoStore, import.meta.hot));
}
