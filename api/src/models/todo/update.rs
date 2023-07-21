use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde_json::json;

use crate::state::AppState;

use super::structs::{TodoCreateUpdate};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(todo): Json<TodoCreateUpdate>,
) -> impl IntoResponse {
    println!("HANDLER: todo_update");

    let collection = state.database.collection::<Document>("todos");
    collection
        .update_one(
            doc! {
              "_id": ObjectId::parse_str(id).unwrap(),
            },
            doc! {
              "$set": {
                "description": todo.description
              }
            },
            None,
        )
        .await
        .unwrap();

    Json(json!({ "status": true }))
}
