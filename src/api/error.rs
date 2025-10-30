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
}
