use axum::response::{Html, IntoResponse};

pub async fn handler_main() -> impl IntoResponse {
    println!("HANDLER: main");

    Html(format!("rust api"))
}
