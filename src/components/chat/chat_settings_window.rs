use crate::api::{
    chat::{UpdateChatRequest, delete_chat, update_chat},
    image::{delete_chat_image, upload_chat_image},
};
use crate::models::chat::Chat;
use crate::utils::{API_BASE_URL, get_current_user_id};
use leptos::prelude::*;
use stylance::import_style;
use web_sys::SubmitEvent;
use web_sys::wasm_bindgen::JsCast;

import_style!(style, "chat_settings_window.module.scss");

#[component]
pub fn ChatSettingsWindow(
    chat: Chat,
    set_show_chat_settings_window: WriteSignal<bool>,
    avatar_count: RwSignal<i32>,
    refetch_chats: Callback<()>,
) -> impl IntoView {
    // SIGNALS
    let chat_id = chat.id;
    let initial_chat_name = chat.name.unwrap_or_default();
    let new_chat_name = RwSignal::new(initial_chat_name.clone());
    let preview_image_url = RwSignal::new(None);
    let selected_file = RwSignal::new_local(None);

    // ACTIONS
    let update_chat_action = Action::new_local(move |req: &UpdateChatRequest| {
        let req = req.clone();
        async move { update_chat(chat_id, req).await }
    });

    let delete_chat_action =
        Action::new_local(move |_: &()| async move { delete_chat(chat_id).await });

    let upload_chat_image_action = Action::new_local(move |_: &()| {
        let file_opt = selected_file.get_untracked().to_owned();
        async move {
            if let Some(file) = file_opt {
                let _ = upload_chat_image(chat_id, file).await;
            }
        }
    });

    let delete_chat_image_action =
        Action::new_local(move |_: &()| async move { delete_chat_image(chat_id).await });

    // EFFECTS
    Effect::new(move |_| {
        if update_chat_action.version().get() > 0
            || delete_chat_action.version().get() > 0
            || upload_chat_image_action.version().get() > 0
            || delete_chat_image_action.version().get() > 0
        {
            *avatar_count.write() += 1;
            refetch_chats.run(());
            set_show_chat_settings_window.set(false);
        }
    });

    // EVENT HANDLERS
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let name = new_chat_name.get_untracked();
        if name != initial_chat_name {
            update_chat_action.dispatch(UpdateChatRequest { new_name: name });
        }
        if selected_file.get().is_some() {
            upload_chat_image_action.dispatch(());
        }
    };

    let on_file_change = move |ev: web_sys::Event| {
        if let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            if let Some(file) = input.files().and_then(|list| list.get(0)) {
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                preview_image_url.set(Some(url));
                selected_file.set(Some(file));
            }
        }
    };

    view! {
        <div class=style::backdrop on:click=move |_| set_show_chat_settings_window.set(false)>
            <div class=style::content on:click=|e| e.stop_propagation()>
                <h2>"Настройки чата"</h2>
                <form on:submit=on_submit>
                    <div class=style::image_upload_section>
                        <div class=style::image_preview>
                            <img src=move || preview_image_url.get().unwrap_or_else(|| format!("{}/chat-image/{}/{}?v={}", API_BASE_URL, get_current_user_id().unwrap_or_default(),chat_id, avatar_count.get())) onerror="this.onerror=null;this.src='/images/chatdefault.webp';"/>
                        </div>
                        <input type="file" accept="image/*" on:change=on_file_change class=style::file_input/>
                        <button type="button" on:click=move |_| {delete_chat_image_action.dispatch(());} class=style::delete_image_button>"Удалить изображение чата"</button>
                    </div>
                    <div class=style::form_field>
                        <label for="chat_name">"Название чата"</label>
                        <input
                            type="text"
                            id="chat_name"
                            bind:value=new_chat_name
                        />
                    </div>
                    <div class=style::form_actions>
                        <button type="submit">"Сохранить"</button>
                        <button type="button" class=style::delete_chat_button on:click=move |_| {delete_chat_action.dispatch(());}>"Удалить чат"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
