use super::{base::ApiClient, error::ApiError};
use crate::models::profile::Profile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    pub new_status: Option<String>,
    pub new_about: Option<String>,
}

pub async fn get_current_user_profile() -> Result<Profile, ApiError> {
    ApiClient::get("/profile/current-user-profile")
        .authenticated()
        .send_json()
        .await
}

pub async fn get_user_profile(user_id: Uuid) -> Result<Profile, ApiError> {
    ApiClient::get(&format!("/profile/{}", user_id))
        .authenticated()
        .send_json()
        .await
}

pub async fn update_profile(request: UpdateProfileRequest) -> Result<Profile, ApiError> {
    ApiClient::patch("/profile/update", &request)
        .authenticated()
        .send_json()
        .await
}
