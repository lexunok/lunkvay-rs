use super::{base::ApiClient, error::ApiError};
use crate::models::user::User;
use uuid::Uuid;

pub async fn get_user_by_id(user_id: Uuid) -> Result<User, ApiError> {
    ApiClient::get(&format!("/user/{}", user_id))
        .authenticated()
        .send_json()
        .await
}
