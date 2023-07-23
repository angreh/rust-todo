use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;

use super::structs::TodoGet;
use crate::global_structs::app_state::AppState;

pub async fn handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    println!("HANDLER: todo_get");

    let collection = state.database.collection::<TodoGet>("todos");

    let todo = collection
        .find_one(
            doc! {
                "_id": ObjectId::parse_str(id).unwrap(),
            },
            None,
        )
        .await
        .unwrap();

    Json(json!({ "success": true, "todo": todo }))
}
