use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde_json::json;

use crate::state::AppState;

pub async fn handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let collection = state.database.collection::<Document>("todos");
    collection
        .delete_one(
            doc! {
              "_id": ObjectId::parse_str(id).unwrap(),
            },
            None,
        )
        .await
        .unwrap();

    Json(json!({ "status": true }))
}
