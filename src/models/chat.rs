use super::user::User;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SystemMessageType {
    #[default]
    None,
    UserJoined,
    UserRejoined,
    UserLeft,
    ChatCreated,
    ChatUpdated,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatMemberRole {
    Member,
    Administrator,
    Owner,
}

#[derive(Clone, Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ChatType {
    Personal = 0,
    Group = 1,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Uuid,
    pub sender: Option<User>,
    pub system_message_type: SystemMessageType,
    pub message: String,
    pub is_edited: bool,
    pub is_pinned: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_my_message: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMember {
    pub id: Uuid,
    pub member: User,
    pub member_name: String,
    pub role: ChatMemberRole,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: Uuid,
    pub name: Option<String>,
    pub last_message: Option<ChatMessage>,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub created_at: NaiveDateTime,
    pub member_count: i32,
}
