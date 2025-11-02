use super::{base::ApiClient, error::ApiError};
use crate::models::chat::{ChatListItem, ChatMessage};
use serde::Serialize;
use uuid::Uuid;

pub async fn get_chat_list() -> Result<Vec<ChatListItem>, ApiError> {
    ApiClient::get("/chats").authenticated().send_json().await
}

pub async fn get_chat_messages(chat_id: Uuid) -> Result<Vec<ChatMessage>, ApiError> {
    ApiClient::get(&format!("/chats/{}", chat_id))
        .authenticated()
        .send_json()
        .await
}

#[derive(Serialize)]
struct SendMessageRequest {
    message: String,
}

pub async fn send_message(chat_id: Uuid, message: String) -> Result<(), ApiError> {
    let body = SendMessageRequest { message };
    ApiClient::post(&format!("/chats/{}/messages", chat_id), &body)
        .authenticated()
        .send_empty()
        .await
}
