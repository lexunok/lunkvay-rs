use super::{base::ApiClient, error::ApiError};
use crate::models::friends::{Friendship, FriendshipLabel, FriendshipStatus};
use crate::models::user::UserListItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFriendshipStatusRequest {
    pub status: FriendshipStatus,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFriendshipLabelRequest {
    pub friendship_id: Uuid,
    pub label: String,
}

// pub async fn get_user_friends(
//     user_id: Uuid,
//     page: Option<u32>,
//     page_size: Option<u32>,
// ) -> Result<Vec<Friendship>, ApiError> {
//     let page = page.unwrap_or(1);
//     let page_size = page_size.unwrap_or(16);
//     let path = format!("/friends/{}?page={}&pageSize={}", user_id, page, page_size);
//     ApiClient::get(&path).authenticated().send_json().await
// }

pub async fn get_friends(
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Friendship>, ApiError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(16);
    let path = format!("/friends?page={}&pageSize={}",page, page_size);
    ApiClient::get(&path).authenticated().send_json().await
}

pub async fn get_incoming_friend_requests(
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Friendship>, ApiError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(16);
    let path = format!("/friends/incoming?page={}&pageSize={}", page, page_size);
    ApiClient::get(&path).authenticated().send_json().await
}

pub async fn get_outgoing_friend_requests(
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Friendship>, ApiError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(16);
    let path = format!("/friends/outgoing?page={}&pageSize={}", page, page_size);
    ApiClient::get(&path).authenticated().send_json().await
}

pub async fn get_possible_friends(
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<UserListItem>, ApiError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(16);
    let path = format!("/friends/possible?page={}&pageSize={}", page, page_size);
    ApiClient::get(&path).authenticated().send_json().await
}

pub async fn send_friend_request(friend_id: Uuid) -> Result<Friendship, ApiError> {
    ApiClient::post(&format!("/friends/{}", friend_id), &())
        .authenticated()
        .send_json()
        .await
}

pub async fn update_friendship_status(
    friendship_id: Uuid,
    request: UpdateFriendshipStatusRequest,
) -> Result<Friendship, ApiError> {
    ApiClient::patch(&format!("/friends/status/{}", friendship_id), &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn get_friendship_labels() -> Result<Vec<FriendshipLabel>, ApiError> {
    ApiClient::get("/friends/labels")
        .authenticated()
        .send_json()
        .await
}

pub async fn create_friendship_label(
    request: CreateFriendshipLabelRequest,
) -> Result<FriendshipLabel, ApiError> {
    ApiClient::post("/friends/labels", &request)
        .authenticated()
        .send_json()
        .await
}

pub async fn delete_friendship_label(friendship_label_id: Uuid) -> Result<(), ApiError> {
    ApiClient::delete(&format!("/friends/labels/{}", friendship_label_id))
        .authenticated()
        .send_empty()
        .await
}

pub async fn delete_friendship_labels_by_label_value(label: String) -> Result<(), ApiError> {
    ApiClient::delete(&format!("/friends/labels?label={}", label))
        .authenticated()
        .send_empty()
        .await
}
