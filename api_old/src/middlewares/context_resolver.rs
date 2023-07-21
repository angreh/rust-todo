use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use mongodb::Database;
use core::result::Result;

use crate::ctx::Ctx;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // parts.extensions.get::<Ctx,Self>().ok_or(Err(())).clone()
        // parts.extensions.get::<Result<Ctx>>().unwrap().clone()
        let xxx = parts.extensions.get::<Ctx>().unwrap();
        println!("{:?}",xxx);
        Ok(Ctx::new(19))
    }
}
pub async fn resolver<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let result_ctx = Ctx::new(8);

    req.extensions_mut().insert(result_ctx);
    println!("kkkkk");

    next.run(req).await
}
