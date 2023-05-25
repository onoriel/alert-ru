use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
    #[serde(skip_serializing)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub updated_at: Option<DateTime<Utc>>,
}