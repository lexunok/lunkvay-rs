use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub is_deleted: bool,
    pub last_login: NaiveDateTime,
    pub is_online: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
    pub user_id: Uuid,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub is_online: bool,
}
