use axum::http::Result;
use axum::middleware;
use axum::response::IntoResponse;
use axum::{http::HeaderValue, routing::get, Router};
use mongodb::{Client, Database};
use tower_http::cors::CorsLayer;
use axum::extract::State;

pub mod todo;

use crate::models::main::handler_main;
use crate::routes::todo::routes as todo_routes;

#[derive(Clone)]
pub struct AppState {
    // database: Database,
}
// impl DbPool {
//     pub async fn new(db: Database) -> Self {
//         Self { database: db }
//     }
// }

pub fn main() -> Router {
    Router::new()
        .merge(todo_routes())
        .route("/", get(handler_main))
        .layer(middleware::from_fn(
            super::middlewares::context_resolver::resolver,
        ))
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        // .with_state(db_pool)
}
