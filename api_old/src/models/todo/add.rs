use axum::{http::StatusCode, extract::State};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
// use mongodb::{bson::doc, bson::Document};
use serde_json::{json, Value};

use crate::{ctx::Ctx, routes::AppState};

// use crate::web::db::connect;
// use super::structs::User;

// pub async fn handler(Json(user): Json<User>) -> impl IntoResponse {
//     println!("HANDLER: todo_add");

//     let database = connect().await;

//     let collection = database.collection::<Document>("users");
//     collection
//         .insert_one(doc! {"name": user.name}, None)
//         .await
//         .unwrap();

//     Json(json!({ "status": true }))
// }

type Result<T> = core::result::Result<T, StatusCode>;
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Xxx {
    pub name: String,
}

pub async fn handler(ctx:Ctx) -> impl IntoResponse {
    println!("HANDLER: todo_add");
    let name = ctx.name();
    println!("{name}oO");

    Json(json!({ "status": true }))
}
