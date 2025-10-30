use leptos::prelude::*;
use web_sys::Storage;

pub fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

pub fn clear_token() {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item("token");
    }
}