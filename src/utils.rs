use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use leptos::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use web_sys::Storage;

pub const DOMAIN: &str = "lunkvay.runasp.net";
pub const API_BASE_URL: &str = "https://lunkvay.lex48949.workers.dev/api/v1";

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub id: Uuid,
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
pub fn has_token() -> bool {
    local_storage()
        .and_then(|storage| storage.get_item("token").ok().flatten())
        .is_some()
}
pub fn clear_token() {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item("token");
    }
}
