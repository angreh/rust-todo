use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tokio::time::Duration;

use crate::global_structs::app_state::AppState;

pub async fn get_state_with_db() -> AppState {
    println!("State: Create DB connection");

    // trvmongo:27017
    let mut client_options =
        ClientOptions::parse("mongodb://root:password@trvmongo:27017/tvr_todo?authSource=admin")
            .await
            .unwrap();
    client_options.connect_timeout = Some(Duration::from_secs(120));
    client_options.max_pool_size = Some(3);
    client_options.min_pool_size = Some(1);
    let client = Client::with_options(client_options).unwrap();

    let database = client.database("tvr_todo");

    database
        .run_command(doc! {"ping":1}, None)
        .await
        .expect("Fail connection with DB!");

    println!("State: Successfully conected!");

    AppState { database }
}
