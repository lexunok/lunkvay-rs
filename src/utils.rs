use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use leptos::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use web_sys::Storage;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub id: Uuid
}

pub fn get_current_user_id() -> Option<Uuid> {
    let storage = local_storage()?;
    let token = storage.get_item("token").ok().flatten()?;

    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.validate_aud = false;

    decode::<Claims>(&token, &DecodingKey::from_secret(&[]), &validation)
        .map(|token_data| token_data.claims.id)
        .ok()
}

pub fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

pub fn clear_token() {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item("token");
    }
}