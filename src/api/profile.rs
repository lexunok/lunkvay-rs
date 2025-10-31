use super::{base::ApiClient, error::ApiError};
use crate::models::profile::UserProfile;

pub async fn get_current_user_profile() -> Result<UserProfile, ApiError> {
    ApiClient::get("/profile/current-user-profile")
        .authenticated()
        .send_json()
        .await
}

pub async fn get_user_profile(user_id: String) -> Result<UserProfile, ApiError> {
    ApiClient::get(&format!("/profile/{}", user_id))
        .authenticated()
        .send_json()
        .await
}
