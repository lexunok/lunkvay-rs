use super::error::ApiError;
use crate::{config::API_BASE_URL, utils::local_storage};
use reqwasm::http::{Method, Request, Response};
use serde::{de::DeserializeOwned, Serialize};

pub struct ApiClient;

impl ApiClient {
    pub fn get<'a>(path: &'a str) -> RequestBuilder<'a, ()> {
        RequestBuilder::new(Method::GET, path, None)
    }

    pub fn post<'a, B: Serialize>(path: &'a str, body: &'a B) -> RequestBuilder<'a, B> {
        RequestBuilder::new(Method::POST, path, Some(body))
    }
}

pub struct RequestBuilder<'a, B: Serialize> {
    method: Method,
    path: &'a str,
    body: Option<&'a B>,
    auth: bool,
}

impl<'a, B: Serialize> RequestBuilder<'a, B> {
    fn new(method: Method, path: &'a str, body: Option<&'a B>) -> Self {
        Self { method, path, body, auth: false }
    }

    pub fn authenticated(mut self) -> Self {
        self.auth = true;
        self
    }

    async fn send_base(self) -> Result<Response, ApiError> {
        let url = format!("{}{}", API_BASE_URL, self.path);
        let mut request_builder = Request::new(&url).method(self.method.clone());

        if self.auth {
            let storage = local_storage().ok_or(ApiError::Network("localStorage не доступен".to_string()))?;
            let token = storage.get_item("token").map_err(|_| ApiError::Network("Не удалось получить токен".to_string()))?.ok_or(ApiError::Unauthorized)?;
            request_builder = request_builder.header("Authorization", &format!("Bearer {}", token));
        }

        if let Some(body_content) = self.body {
            let request_body = serde_json::to_string(body_content).map_err(|e| ApiError::Parsing(e.to_string()))?;
            request_builder = request_builder.header("Content-Type", "application/json").body(request_body);
        }

        request_builder.send().await.map_err(|e| ApiError::Network(e.to_string()))
    }

    pub async fn send_json<T: DeserializeOwned>(self) -> Result<T, ApiError> {
        let response = self.send_base().await?;
        if !response.ok() {
            return Err(ApiError::from_response(response).await);
        }
        response.json::<T>().await.map_err(|e| ApiError::Parsing(e.to_string()))
    }

    pub async fn send_text(self) -> Result<String, ApiError> {
        let response = self.send_base().await?;
        if !response.ok() {
            return Err(ApiError::from_response(response).await);
        }
        response.text().await.map_err(|e| ApiError::Parsing(e.to_string()))
    }

    pub async fn send_empty(self) -> Result<(), ApiError> {
        let response = self.send_base().await?;
        if !response.ok() {
            return Err(ApiError::from_response(response).await);
        }
        Ok(())
    }
}