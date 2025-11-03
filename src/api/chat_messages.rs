use super::{base::ApiClient, error::ApiError};
use crate::models::chat::ChatMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatMessageRequest {
    pub chat_id: Uuid,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEditChatMessageRequest {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub new_message: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePinChatMessageRequest {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub is_pinned: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatMessageRequest {
    pub message_id: Uuid,
    pub chat_id: Uuid,
}

pub async fn get_chat_messages(
    chat_id: Uuid,
    pinned: Option<bool>,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<ChatMessage>, ApiError> {
    let pinned = pinned.unwrap_or(false);
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(16);
    let path = format!(
        "/chats/messages/{}?page={}&pageSize={}&pinned={}",
        chat_id, page, page_size, pinned
    );
    ApiClient::get(&path).authenticated().send_json().await
}

pub async fn create_chat_message(
    request: CreateChatMessageRequest,
) -> Result<ChatMessage, ApiError> {
    ApiClient::post("/chats/messages", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn update_edit_chat_message(
    request: UpdateEditChatMessageRequest,
) -> Result<ChatMessage, ApiError> {
    ApiClient::patch("/chats/messages/edit", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn update_pin_chat_message(
    request: UpdatePinChatMessageRequest,
) -> Result<ChatMessage, ApiError> {
    ApiClient::patch("/chats/messages/pin", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn delete_chat_message(request: DeleteChatMessageRequest) -> Result<(), ApiError> {
    ApiClient::delete_with_body("/chats/messages", &request)
        .authenticated()
        .send_json()
        .await
}
