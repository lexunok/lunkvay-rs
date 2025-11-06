use crate::api::{profile::{update_profile}, image::{delete_avatar, upload_avatar}};
use leptos::prelude::*;
use stylance::import_style;
use web_sys::SubmitEvent;
use crate::api::profile::UpdateProfileRequest;
use web_sys::wasm_bindgen::JsCast;
import_style!(style, "editing_window.module.scss");

#[component]
pub fn EditingWindow(
    status: String, 
    about: String, 
    avatar_url:String, 
    set_show_editing_window: WriteSignal<bool>, 
    refetch_profile: Callback<()>
) -> impl IntoView {
    //SIGNALS
    let new_status = RwSignal::new(status);
    let new_about = RwSignal::new(about);
    let preview_url = RwSignal::new(None);
    let selected_file = RwSignal::new_local(None);

    //ACTIONS
    let delete_avatar_action = Action::new_local(move |_: &()| async move { delete_avatar().await });
    
    let update_profile_action = Action::new_local(|req: &UpdateProfileRequest| {
        let req = req.clone();
        async move {
            update_profile(req)
                .await
                .map_err(|e| e.to_string())
        }
    });

    let upload_avatar_action = Action::new_local(move |_: &()| {
        let file_opt = selected_file.get_untracked().to_owned();
        async move {
            if let Some(file) = file_opt {
                let _ = upload_avatar(file).await.map_err(|e| e.to_string());
            }
        }
    });

    //EVENTS
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let request = UpdateProfileRequest {
            new_status: Some(new_status.get_untracked()),
            new_about: Some(new_about.get_untracked()),
        };
        update_profile_action.dispatch(request);
        if let Some(_) = selected_file.get() {
            upload_avatar_action.dispatch(());
        }
        set_show_editing_window.set(false);
    };
    let on_file_change = move |ev: web_sys::Event| {
        if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            if let Some(file) = input.files().and_then(|list| list.get(0)) {
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                preview_url.set(Some(url));
                selected_file.set(Some(file));
            }
        }
    };
    
    //EFFECTS
    Effect::new(move |_| {
        if update_profile_action.version().get() > 0
            || upload_avatar_action.version().get() > 0
            || delete_avatar_action.version().get() > 0
        {
            refetch_profile.run(());
        }
    });

    //VIEW
    view! {
        <div class=style::backdrop on:click=move |_| set_show_editing_window.set(false)>
            <div class=style::content on:click=|e| e.stop_propagation()>
                <h2>"Редактировать профиль"</h2>
                <form on:submit=on_submit>
                    <div class=style::avatar_upload_section>
                        <div class=style::avatar_preview>
                            <img src=move || preview_url.get().unwrap_or_else(|| avatar_url.clone()) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                        </div>
                        <input type="file" accept="image/*" on:change=on_file_change class=style::file_input/>
                        <button type="button" on:click=move |_| {delete_avatar_action.dispatch(());} class=style::delete_avatar_button>"Удалить аватар"</button>
                    </div>
                    <div class=style::form_field>
                        <label for="status">"Статус"</label>
                        <input
                            type="text"
                            id="status"
                            bind:value=new_status
                        />
                    </div>
                    <div class=style::form_field>
                        <label for="about">"О себе"</label>
                        <textarea
                            id="about"
                            bind:value=new_about
                            rows="5"
                        ></textarea>
                    </div>
                    <div class=style::form_actions>
                        <button type="submit">"Сохранить"</button>
                        <button type="button" class=style::cancel_button on:click=move |_| set_show_editing_window.set(false)>"Отмена"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
