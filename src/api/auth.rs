use crate::config::API_BASE_URL;
use crate::models::auth::{LoginRequest, RegisterRequest};
use reqwasm::http::{Method, Request};

// Define a custom error type for API errors to avoid propagating reqwasm::Error
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestError(String),
    #[error("Server returned an error: {0}")]
    ServerError(String),
}

// Helper to convert reqwasm::Error to our ApiError
impl From<reqwasm::Error> for ApiError {
    fn from(err: reqwasm::Error) -> Self {
        ApiError::RequestError(err.to_string())
    }
}

pub async fn login(creds: LoginRequest<'_>) -> Result<String, ApiError> {
    let request_body = serde_json::to_string(&creds).unwrap(); // In real app, handle error
    let request = Request::new(&format!("{}/auth/login", API_BASE_URL))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(request_body);

    match request.send().await {
        Ok(response) if response.ok() => {
            // Assuming the token is returned as plain text in the body
            response.text().await.map_err(|e| ApiError::RequestError(e.to_string()))
        }
        Ok(response) => {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown server error".to_string());
            Err(ApiError::ServerError(error_text))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn register(details: RegisterRequest<'_>) -> Result<(), ApiError> {
    let request_body = serde_json::to_string(&details).unwrap(); // In real app, handle error
    let request = Request::new(&format!("{}/auth/register", API_BASE_URL))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(request_body);

    match request.send().await {
        Ok(response) if response.ok() => Ok(()),
        Ok(response) => {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown server error".to_string());
            Err(ApiError::ServerError(error_text))
        }
        Err(e) => Err(e.into()),
    }
}
