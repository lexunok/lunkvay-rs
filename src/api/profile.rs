use crate::{config::API_BASE_URL, models::profile::UserProfile, utils::local_storage};
use super::error::ApiError;
use reqwasm::http::Request;

pub async fn get_current_user_profile() -> Result<UserProfile, ApiError> {
    let storage = local_storage().ok_or(ApiError::Network("localStorage не доступен".to_string()))?;
    let token = storage
        .get_item("token")
        .map_err(|_| ApiError::Network("Не удалось получить токен".to_string()))?
        .ok_or(ApiError::Unauthorized)?;

    let url = format!("{}/profile/current-user-profile", API_BASE_URL);

    let response_result = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await;

    match response_result {
        Ok(response) => {
            if response.status() == 401 {
                return Err(ApiError::Unauthorized);
            }
            if !response.ok() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Не удалось получить текст ошибки".to_string());
                return Err(ApiError::Server(format!(
                    "Ошибка API: {} - {}",
                    response.status(),
                    error_text
                )));
            }

            response
                .json::<UserProfile>()
                .await
                .map_err(|e| ApiError::Parsing(e.to_string()))
        }
        Err(e) => {
            Err(ApiError::Network(e.to_string()))
        }
    }
}
