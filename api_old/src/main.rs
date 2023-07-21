// #[macro_use]
// extern crate lazy_static;

use std::net::SocketAddr;

use axum::extract::FromRef;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::middleware;
use axum::routing::get;
use axum::Router;
use mongodb::Client;
use mongodb::Database;
use tower_http::cors::CorsLayer;

static mut DB: Option<Database> = None;
// static mut DB: Dattabase = Option<Mutex<Data>;
// lazy_static! {
//     static ref CON: Database = Some(Mutex::new());
// }

mod middlewares;
mod models;
mod routes;

mod ctx;

#[derive(Clone)]
pub struct AppState {
    database: Database,
}

#[tokio::main]
async fn main() {
    // let client =
    //     Client::with_uri_str("mongodb://root:password@localhost:27017/fluke_lms?authSource=admin")
    //         .await
    //         .expect("Failed to connect to MongoDB");

    // let database = client.database("fluke_lms");
    let db_pool = AppState {};
    let routes = Router::new()
        .merge(routes::todo::routes())
        .route("/", get(models::main::handler_main))
        // .layer(middleware::from_fn(middlewares::context_resolver::resolver))
        // .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        .with_state(db_pool);

    println!("hmm");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on http://localhost:8080\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    let client =
        Client::with_uri_str("mongodb://root:password@localhost:27017/tvr_todo?authSource=admin")
            .await
            .expect("Failed to connect to MongoDB");
    unsafe {
        DB = Some(client.database("tvr_todo"));
    }
}
