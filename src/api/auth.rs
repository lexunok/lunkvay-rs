use crate::config::API_BASE_URL;
use crate::models::auth::{LoginRequest, RegisterRequest};
use super::error::ApiError;
use reqwasm::http::{Method, Request};

pub async fn login(creds: LoginRequest) -> Result<String, ApiError> {
    let request_body = serde_json::to_string(&creds)
        .map_err(|e| ApiError::Parsing(e.to_string()))?;

    let request = Request::new(&format!("{}/auth/login", API_BASE_URL))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(request_body);

    match request.send().await {
        Ok(response) => {
            if response.status() == 401 {
                return Err(ApiError::Unauthorized);
            }
            if !response.ok() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Не удалось получить текст ошибки".to_string());
                return Err(ApiError::Server(error_text));
            }
            response.text().await.map_err(|e| ApiError::Parsing(e.to_string()))
        }
        Err(e) => Err(ApiError::Network(e.to_string())),
    }
}

pub async fn register(details: RegisterRequest) -> Result<(), ApiError> {
    let request_body = serde_json::to_string(&details)
        .map_err(|e| ApiError::Parsing(e.to_string()))?;

    let request = Request::new(&format!("{}/auth/register", API_BASE_URL))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(request_body);

    match request.send().await {
        Ok(response) => {
            if !response.ok() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Не удалось получить текст ошибки".to_string());
                return Err(ApiError::Server(error_text));
            }
            Ok(())
        }
        Err(e) => Err(ApiError::Network(e.to_string())),
    }
}
