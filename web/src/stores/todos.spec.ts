// https://testdriven.io/blog/vue-pinia-testing/

import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useTodoStore } from "./todos";

// mock attempt
import { create } from "../services/todo";
vi.mock("../services/todo", () => {
  return {
    create: vi.fn(() => null),
  };
});
// @ts-ignore
create.mockImplementation((desc: any) => {
  return {
    data: {
      id: {
        $oid: "somemongoid",
      },
    },
  };
});

// for wait the settimeout in the function
function timeout(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("todos store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("test adding a new todo", async () => {
    const store = useTodoStore();
    await store.todo_create("algo ali");

    // this is needed in case of failure
    await timeout(1500);

    console.log(store.todos);

    expect(store.todos.length).toEqual(1);
  });
});
