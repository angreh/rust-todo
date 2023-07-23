use axum::routing::{delete, get};
use axum::Router;

use crate::models::todo::{create, delete, get, list, update};
use crate::global_structs::app_state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list::handler).post(create::handler))
        .route(
            "/todos/:id",
            delete(delete::handler)
                .patch(update::handler)
                .get(get::handler),
        )
        .with_state(state)
}
