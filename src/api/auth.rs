use super::base::ApiClient;
use super::error::ApiError;
use crate::models::auth::{LoginRequest, RegisterRequest};

pub async fn login(creds: LoginRequest) -> Result<String, ApiError> {
    ApiClient::post("/auth/login", &creds).send_text().await
}

pub async fn register(details: RegisterRequest) -> Result<(), ApiError> {
    ApiClient::post("/auth/register", &details)
        .send_empty()
        .await
}
