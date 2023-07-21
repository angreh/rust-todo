import { ref, computed } from "vue";
import { defineStore, acceptHMRUpdate } from "pinia";
// import { call } from "../services/todo";

export const useCounterStore = defineStore("counter", () => {
  const count = ref(0);
  const doubleCount = computed(() => count.value * 2);

  function increment() {
    // call()
    count.value++;
  }

  return { count, doubleCount, increment };
});

// make sure to pass the right store definition, `useAuth` in this case.
if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useCounterStore, import.meta.hot));
}
