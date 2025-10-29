use leptos::prelude::*;
use web_sys::Storage;

pub fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}