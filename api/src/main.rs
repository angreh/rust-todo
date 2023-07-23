use std::net::SocketAddr;

mod configs;
mod ctx;
mod middlewares;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = configs::get_state_with_db().await;

    let routes = routes::main(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Listening on http://localhost:8080\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
