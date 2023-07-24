use axum::{extract::State, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde_json::{json, Value};

use super::structs::TodoGet;
use crate::global_structs::{api_error::ApiError, api_result::ApiResult, app_state::AppState};

pub async fn handler(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    println!("HANDLER: todo_list");

    let collection = state.database.collection::<TodoGet>("todos");

    let cursor = match collection.find(doc! {}, None).await {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailNoDbConnection),
    };
    let todos: Vec<TodoGet> = match cursor.try_collect().await {
        Ok(f) => f,
        Err(_) => return Err(ApiError::InternalError),
    };

    Ok(Json(json!({ "success": true, "list": todos })))
}
