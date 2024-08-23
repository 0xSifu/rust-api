use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Follow {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub follower_id: ObjectId,
    pub following_id: ObjectId,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
