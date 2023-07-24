import axios from "axios";

const URL = "http://localhost:8080/todos";

export async function update(item: any) {
  return await axios.patch(`${URL}/` + item.id, {
    description: item.description,
  });
}

export async function list() {
  return await axios.get(URL);
}

export async function remove(id: string) {
  return await axios.delete(`${URL}/` + id);
}

export async function get(id: string) {
  return await axios.get(`${URL}/` + id);
}

export async function create(description: string) {
  return await axios.post(URL, { description });
}
