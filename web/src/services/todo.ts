import axios from "axios";

export async function update(item: any) {
  return await axios.patch("http://localhost:8080/todos/" + item.id, {
    description: item.description,
  });
}

export async function list() {
  return await axios.get("http://localhost:8080/todos");
}

export async function remove(id: string) {
  return await axios.delete("http://localhost:8080/todos/" + id);
}

export async function get(id: string) {
  return await axios.get("http://localhost:8080/todos/" + id);
}

export async function create(description: string) {
  return await axios.post("http://localhost:8080/todos", { description });
}
