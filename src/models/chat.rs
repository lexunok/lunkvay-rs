use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize_repr, Serialize_repr, Default)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum SystemMessageType {
    #[default]
    None = 0,
    UserJoined = 1,
    UserRejoined = 2,
    UserLeft = 3,
    ChatCreated = 4,
    ChatUpdated = 5,
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
    pub sender_id: Option<Uuid>,
    pub sender_user_name: Option<String>,
    pub sender_first_name: Option<String>,
    pub sender_last_name: Option<String>,
    pub sender_is_online: Option<bool>,
    pub system_message_type: SystemMessageType,
    pub message: String,
    pub is_edited: bool,
    pub is_pinned: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub pinned_at: Option<NaiveDateTime>,
    pub is_my_message: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMember {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub member_name: String,
    pub is_online: bool,
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum WsMessageType {
    ReceiveMessage,
    MessageUpdated,
    MessageDeleted,
    MessagePinned,
    MemberUpdated,
    MemberDeleted,
    ChatUpdated,
    ChatDeleted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedMessageData {
    pub message_id: Uuid,
    pub is_pinned: bool,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub r#type: WsMessageType,
    pub data: serde_json::Value,
}
