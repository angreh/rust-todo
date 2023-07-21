use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoGet {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoCreateUpdate {
    pub description: String,
}
