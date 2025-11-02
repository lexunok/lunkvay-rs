use super::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SystemMessageType {
    #[default]
    None,
    UserJoined,
    UserLeft,
    ChatCreated,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    #[serde(default)]
    pub sender: Option<User>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub system_message_type: SystemMessageType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatType {
    Private,
    Group,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatListItem {
    pub id: Uuid,
    pub name: String,
    pub chat_type: ChatType,
    pub last_message: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub unread_count: i32,
    pub avatar_url: Option<String>,
}