import axios from "axios";

export async function list() {
  return await axios.get("http://localhost:8080/todos");
}

export async function create(description: string) {
  return await axios.post(
    "http://localhost:8080/todos",
    { description },
    {
      headers: {
        "Content-Type": "application/json",
      },
    }
  );
}
