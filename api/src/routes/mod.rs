use axum::http::Method;
use axum::{http::HeaderValue, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::models::main::handler_main;
use crate::state::AppState;

pub mod todo;
pub mod user;

pub fn main(state: AppState) -> Router {
    Router::new()
        .merge(todo::routes(state.clone()))
        .merge(user::routes(state.clone()))
        .route("/", get(handler_main))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_origin("*".parse::<HeaderValue>().unwrap()),
        )
}
