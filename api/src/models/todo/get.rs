use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::{json, Value};

use super::structs::TodoGet;
use crate::global_structs::{api_error::ApiError, api_result::ApiResult, app_state::AppState};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Value>> {
    println!("HANDLER: todo_get");

    let collection = state.database.collection::<TodoGet>("todos");

    let _id = match ObjectId::parse_str(id.clone()) {
        Ok(f) => f,
        Err(_) => return Err(ApiError::ResourceActionFailInvalidId { id }),
    };

    let todo = match collection
        .find_one(
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

    if todo == None {
        return Err(ApiError::ResourceActionFailIdNotFound { id });
    }

    Ok(Json(json!({ "success": true, "todo": todo })))
}
