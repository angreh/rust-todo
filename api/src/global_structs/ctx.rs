use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Response;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Ctx {
    pub name: String,
    pub user: User,
}

impl Ctx {
    pub fn new(name: String) -> Self {
        Self {
            name,
            user: User {
                name: String::from("Angreh"),
                id: String::from("ID01"),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub id: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("extractor-ish");

        let ctx = parts.extensions.get::<Ctx>().unwrap().clone();

        Ok(ctx)
    }
}
