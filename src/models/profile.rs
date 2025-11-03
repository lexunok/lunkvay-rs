use super::user::{User, UserListItem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    pub user: User,
    pub status: Option<String>,
    pub about: Option<String>,
    pub friends_count: i32,
    pub friends: Vec<UserListItem>,
}
