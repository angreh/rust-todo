use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
// use mongodb::bson::Document;
use serde_json::json;

use super::structs::TodoCreateUpdate;
use crate::global_structs::app_state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Json(todo): Json<TodoCreateUpdate>,
) -> impl IntoResponse {
    println!("HANDLER: todo_create");

    let collection = state.database.collection::<Document>("todos");
    let result = collection
        .insert_one(doc! {"description": todo.description}, None)
        .await
        .unwrap();

    Json(json!({ "success": true, "id": result.inserted_id }))
}
