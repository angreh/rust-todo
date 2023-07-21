// use mongodb::{Client, Database};

// use crate::DB;

// pub async fn connect() -> Database {
//     unsafe {
//         println!("{DB}");
//     }
//     let client =
//         Client::with_uri_str("mongodb://root:password@localhost:27017/tvr_todo?authSource=admin")
//             .await
//             .expect("Failed to connect to MongoDB");
//     client.database("tvr_todo")
// }
