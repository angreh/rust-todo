use axum::{extract::Path, extract::State, Json};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde_json::{json, Value};

use crate::global_structs::{api_error::ApiError, api_result::ApiResult, app_state::AppState};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Value>> {
    println!("HANDLER: todo_delete");

    let _id = match ObjectId::parse_str(id.clone()) {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceDeleteFailInvalidId { id }),
    };

    let collection = state.database.collection::<Document>("todos");
    let result = match collection
        .delete_one(
            doc! {
              "_id": _id,
            },
            None,
        )
        .await
    {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailNoDbConnection),
    };

    if result.deleted_count == 0 {
        return Err(ApiError::ResourceDeleteFailIdNotFound { id });
    }

    Ok(Json(json!({ "success": true })))
}
