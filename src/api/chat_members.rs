use super::{base::ApiClient, error::ApiError};
use crate::models::chat::{ChatMember, ChatMemberRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatMemberRequest {
    pub chat_id: Uuid,
    pub member_id: Uuid,
    pub inviter_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatMemberRequest {
    pub chat_id: Uuid,
    pub member_id: Uuid,
    pub new_member_name: Option<String>,
    pub new_role: Option<ChatMemberRole>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatMemberRequest {
    pub chat_id: Uuid,
    pub member_id: Uuid,
}

pub async fn get_chat_members(chat_id: Uuid) -> Result<Vec<ChatMember>, ApiError> {
    ApiClient::get(&format!("/chats/members/{}", chat_id))
        .authenticated()
        .send_json()
        .await
}

pub async fn create_chat_member(request: CreateChatMemberRequest) -> Result<ChatMember, ApiError> {
    ApiClient::post("/chats/members", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn update_chat_member(request: UpdateChatMemberRequest) -> Result<ChatMember, ApiError> {
    ApiClient::patch("/chats/members", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn delete_chat_member(request: DeleteChatMemberRequest) -> Result<(), ApiError> {
    ApiClient::delete_with_body("/chats/members", &request)
        .authenticated()
        .send_empty()
        .await
}
