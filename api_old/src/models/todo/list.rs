use axum::response::IntoResponse;
use axum::Json;
// use mongodb::{bson::doc, bson::Document};
use serde_json::json;

// use crate::web::db::connect;
// use super::structs::User;

pub async fn handler() -> impl IntoResponse {
    println!("HANDLER: todo_list");

    let database = connect().await;

    let collection = database.collection::<Document>("users");
    collection
        .insert_one(doc! {"name": user.name}, None)
        .await
        .unwrap();

    Json(json!({ "status": true }))
}

