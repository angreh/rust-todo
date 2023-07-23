use axum::response::{Html, IntoResponse};

pub async fn handler_main() -> impl IntoResponse {
    Html(format!("todo api"))
}
