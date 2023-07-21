use axum::routing::get;
use axum::Router;

use crate::models::todo::{add};

pub fn routes() -> Router {
    Router::new().route("/todos", get(add::handler))
}
