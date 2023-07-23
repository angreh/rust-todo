use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde_json::json;

use super::structs::TodoCreateUpdate;
use crate::state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Json(todo): Json<TodoCreateUpdate>,
) -> impl IntoResponse {
    println!("HANDLER: todo_create");

    let collection = state.database.collection::<Document>("todos");
    collection
        .insert_one(doc! {"description": todo.description}, None)
        .await
        .unwrap();

    Json(json!({ "status": true }))
}
