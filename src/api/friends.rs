use super::{base::ApiClient, error::ApiError};
use crate::models::user::UserListItem;
use uuid::Uuid;

pub async fn get_user_friends(user_id: Uuid) -> Result<Vec<UserListItem>, ApiError> {
    ApiClient::get(&format!("/friends/{}", user_id))
        .authenticated()
        .send_json()
        .await
}
