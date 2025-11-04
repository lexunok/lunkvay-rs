use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FriendshipStatus {
    Pending = 0,
    Accepted = 1,
    Rejected = 2,
    Cancelled = 3,
    Deleted = 4,
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
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub is_online: bool,
}
