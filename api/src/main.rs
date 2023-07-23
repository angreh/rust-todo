use std::net::SocketAddr;

mod configs;
mod routes;
mod state;
mod models;

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
