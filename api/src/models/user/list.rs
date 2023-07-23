use axum::{extract::State, response::IntoResponse, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde_json::json;

use crate::ctx::Ctx;
use crate::models::todo::structs::TodoGet;
use crate::state::AppState;

pub async fn handler(State(state): State<AppState>, ctx: Ctx) -> impl IntoResponse {
    println!("HANDLER: user_list");
    println!("ctx: {},{},{}", ctx.name, ctx.user.name, ctx.user.id);

    let collection = state.database.collection::<TodoGet>("todos");

    let cursor = collection.find(doc! {}, None).await.unwrap();
    let todos: Vec<TodoGet> = cursor.try_collect().await.unwrap();

    Json(json!({ "status": true, "list": todos }))
}
