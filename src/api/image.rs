use super::{base::ApiClient, error::ApiError};
use uuid::Uuid;
use web_sys::FormData;

pub async fn upload_avatar(file: web_sys::File) -> Result<String, ApiError> {
    let form_data = FormData::new().unwrap();
    form_data
        .append_with_blob_and_filename("avatarFile", &file, &file.name())
        .unwrap();

    let response = ApiClient::post_form_data("/avatar", form_data)
        .authenticated()
        .send_base()
        .await?;

    if !response.ok() {
        return Err(ApiError::from_response(response).await);
    }
    response
        .text()
        .await
        .map_err(|e| ApiError::Parsing(e.to_string()))
}
pub async fn delete_avatar() -> Result<String, ApiError> {
    ApiClient::delete("/avatar")
        .authenticated()
        .send_text()
        .await
}
pub async fn upload_chat_image(chat_id: Uuid, file: web_sys::File) -> Result<String, ApiError> {
    let form_data = FormData::new().unwrap();
    form_data
        .append_with_blob_and_filename("avatarFile", &file, &file.name())
        .unwrap();

    let response = ApiClient::post_form_data(&format!("/chat-image/{}", chat_id), form_data)
        .authenticated()
        .send_base()
        .await?;

    if !response.ok() {
        return Err(ApiError::from_response(response).await);
    }
    response
        .text()
        .await
        .map_err(|e| ApiError::Parsing(e.to_string()))
}

pub async fn delete_chat_image(chat_id: Uuid) -> Result<String, ApiError> {
    ApiClient::delete(&format!("/chat-image/{}", chat_id))
        .authenticated()
        .send_text()
        .await
}
