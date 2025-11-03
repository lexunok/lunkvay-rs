use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipLabel {
    pub id: Uuid,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Friendship {
    pub friendship_id: Uuid,
    pub status: Option<FriendshipStatus>,
    pub labels: Option<Vec<FriendshipLabel>>,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub is_online: bool,
}
