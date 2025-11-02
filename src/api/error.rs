use leptos_router::params::ParamsError;
use reqwasm::http::Response;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ApiError {
    #[error("Пользователь не авторизован")]
    Unauthorized,
    #[error("Ошибка сети: {0}")]
    Network(String),
    #[error("Ошибка парсинга: {0}")]
    Parsing(String),
    #[error("Ошибка сервера: {0}")]
    Server(String),
    #[error("Не найдено")]
    NotFound,
}

impl From<ParamsError> for ApiError {
    fn from(_: ParamsError) -> Self {
        ApiError::NotFound
    }
}

impl ApiError {
    pub async fn from_response(response: Response) -> Self {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Не удалось получить текст ошибки".to_string());
        ApiError::Server(error_text)
    }
}
