use axum::routing::get;
use axum::Router;
use axum::middleware;

use crate::models::user::list;
use crate::global_structs::app_state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/users", get(list::handler))
        .layer(middleware::from_fn(
          crate::middlewares::context_resolver::require_auth,
        ))
        .with_state(state)
}
