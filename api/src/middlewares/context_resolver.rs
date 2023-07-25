use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::global_structs::ctx::Ctx;

pub async fn require_auth<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    println!("require_auth");

    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_value| auth_value.to_str().ok())
        .unwrap();
    let result_ctx = Ctx::new(token.to_owned());

    req.extensions_mut().insert(result_ctx);

    next.run(req).await
}
