use axum::{extract::Path, extract::State, Json};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde_json::{json, Value};

use crate::global_structs::{api_error::ApiError, api_result::ApiResult, app_state::AppState};

use super::structs::TodoCreateUpdate;

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(todo): Json<TodoCreateUpdate>,
) -> ApiResult<Json<Value>> {
    println!("HANDLER: todo_update");

    let _id = match ObjectId::parse_str(id.clone()) {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailInvalidId { id }),
    };

    let collection = state.database.collection::<Document>("todos");
    let result = match collection
        .update_one(
            doc! {
              "_id": _id,
            },
            doc! {
              "$set": {
                "description": todo.description
              }
            },
            None,
        )
        .await
    {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailNoDbConnection),
    };

    if result.matched_count == 0 {
        return Err(ApiError::ResourceActionFailIdNotFound { id });
    }

    Ok(Json(json!({ "success": true })))
}
