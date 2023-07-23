use axum::{extract::State, response::IntoResponse, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde_json::json;

use super::structs::TodoGet;
use crate::state::AppState;

pub async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    println!("HANDLER: todo_list");

    let collection = state.database.collection::<TodoGet>("todos");

    let cursor = collection.find(doc! {}, None).await.unwrap();
    let todos: Vec<TodoGet> = cursor.try_collect().await.unwrap();

    Json(json!({ "status": true, "list": todos }))
}
