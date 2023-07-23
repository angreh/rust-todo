use std::net::SocketAddr;

use axum::{Json, middleware};
use axum::response::{Response, IntoResponse};
use serde_json::json;

use crate::global_structs::api_error::ApiError;

mod configs;
mod global_structs;
mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let state = configs::get_state_with_db().await;

    let routes = routes::main(state).layer(middleware::map_response(main_response_wrapper));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Listening on http://localhost:8080\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn main_response_wrapper(
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let service_error = res.extensions().get::<ApiError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // if client error, build the response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "success": false,
                "error": {
                    "type": client_error.as_ref(),
                }
            });

            (*status_code, Json(client_error_body)).into_response()
        });

    println!("server error: {service_error:?}");
    // let client_error = client_status_error.unzip().1;
    // log_request(uuid, req_method, uri, ctx, service_error, client_error).await;


    println!();
    error_response.unwrap_or(res)
}