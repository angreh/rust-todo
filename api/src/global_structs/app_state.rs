use axum::extract::FromRef;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}
impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.database.clone()
    }
}
