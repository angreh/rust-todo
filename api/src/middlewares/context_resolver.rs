use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use core::result::Result;

use crate::ctx::Ctx;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("extractor-ish");

        let ctx = parts.extensions.get::<Ctx>().unwrap().clone();

        Ok(ctx)
    }
}

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
