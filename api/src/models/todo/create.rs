use axum::{extract::State, Json};
use mongodb::bson::{doc, Document};
use serde_json::{json, Value};

use super::structs::TodoCreateUpdate;
use crate::global_structs::{api_error::ApiError, api_result::ApiResult, app_state::AppState};

pub async fn handler(
    State(state): State<AppState>,
    Json(todo): Json<TodoCreateUpdate>,
) -> ApiResult<Json<Value>> {
    println!("HANDLER: todo_create");

    let collection = state.database.collection::<Document>("todos");
    let result = match collection
        .insert_one(doc! {"description": todo.description}, None)
        .await
    {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailNoDbConnection),
    };

    Ok(Json(json!({ "success": true, "id": result.inserted_id })))
}
