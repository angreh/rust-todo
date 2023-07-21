use mongodb::Client;

use super::state::AppState;

pub async fn get_state_with_db() -> AppState {
    let client =
        Client::with_uri_str("mongodb://root:password@localhost:27017/tvr_todo?authSource=admin")
            .await
            .expect("Failed to connect to MongoDB");

    let database = client.database("tvr_todo");
    AppState { database }
}
