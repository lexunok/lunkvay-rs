use super::{base::ApiClient, error::ApiError};
use crate::models::chat::Chat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalChatRequest {
    pub interlocutor: Uuid,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupChatRequest {
    pub name: String,
    pub members: Vec<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatRequest {
    pub new_name: String,
}

pub async fn get_all_chats() -> Result<Vec<Chat>, ApiError> {
    ApiClient::get("/chats").authenticated().send_json().await
}

pub async fn create_personal_chat(request: CreatePersonalChatRequest) -> Result<Chat, ApiError> {
    ApiClient::post("/chats/personal", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn create_group_chat(request: CreateGroupChatRequest) -> Result<Chat, ApiError> {
    ApiClient::post("/chats/group", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn update_chat(chat_id: Uuid, request: UpdateChatRequest) -> Result<Chat, ApiError> {
    ApiClient::patch(&format!("/chats/{}", chat_id), &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn delete_chat(chat_id: Uuid) -> Result<(), ApiError> {
    ApiClient::delete(&format!("/chats/{}", chat_id))
        .authenticated()
        .send_empty()
        .await
}
