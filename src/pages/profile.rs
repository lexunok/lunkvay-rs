use crate::{
    api::{self, image::{delete_avatar, upload_avatar}},
    components::{friend_card::FriendCard, spinner::Spinner},
    utils::{API_BASE_URL, get_current_user_id},
};
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use stylance::import_style;
use uuid::Uuid;
use web_sys::File;
use web_sys::wasm_bindgen::JsCast;
import_style!(style, "profile.module.scss");

#[derive(Params, PartialEq, Clone, Debug)]
struct ProfileParams {
    id: Option<Uuid>,
}

use crate::api::profile::UpdateProfileRequest;
use leptos::ev::SubmitEvent;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let (show_modal, set_show_modal) = signal(false);
    let (selected_file, set_selected_file) = signal_local::<Option<File>>(None);
    let (preview_url, set_preview_url) = signal::<Option<String>>(None);

    let update_profile_action = Action::new_local(|req: &UpdateProfileRequest| {
        let req = req.clone();
        async move {
            api::profile::update_profile(req)
                .await
                .map_err(|e| e.to_string())
        }
    });

    let upload_avatar_action = Action::new_local(move|_: &()| {
        let file_opt = selected_file.get().to_owned();
        async move {
            if let Some(file) = file_opt {
                let _ = upload_avatar(file).await.map_err(|e| e.to_string());
            }
        }
    });

    let delete_avatar_action = Action::new_local(|_: &()| async move { delete_avatar().await });

    let profile_resource = LocalResource::new(move || {
        let params = params.get();
        async move {
            let params = params?;
            if let Some(user_id) = params.id {
                api::profile::get_user_profile(user_id).await
            } else {
                api::profile::get_current_user_profile().await
            }
        }
    });

    Effect::new(move |_| {
        if update_profile_action.version().get() > 0
            || upload_avatar_action.version().get() > 0
            || delete_avatar_action.version().get() > 0
        {
            profile_resource.refetch();
        }
    });

    let on_file_change = move |ev: web_sys::Event| {
        let input = ev.target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>(); 
        if let Some(file_list) = input.files() {
            if let Some(file) = file_list.get(0) {
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                set_preview_url.set(Some(url));
                set_selected_file.set(Some(file));
            }
        }
    };

    let profile_view = move || {
        profile_resource.get().map(|result| match result {
            Ok(profile) => {
                let (avatar_url, _) = signal(format!(
                    "{}/avatar/{}",
                    API_BASE_URL,
                    profile.user.id
                ));
                let is_current_user = get_current_user_id().map_or(false, |id| id == profile.user.id);

                let new_status = RwSignal::new(profile.status.clone().unwrap_or_default());
                let new_about = RwSignal::new(profile.about.clone().unwrap_or_default());

                let on_submit = move |ev: SubmitEvent| {
                    ev.prevent_default();
                    let request = UpdateProfileRequest {
                        new_status: Some(new_status.get()),
                        new_about: Some(new_about.get()),
                    };
                    update_profile_action.dispatch(request);
                    if let Some(_) = selected_file.get() {
                        upload_avatar_action.dispatch(());
                    }
                    set_show_modal.set(false);
                };

                view! {
                    <div class=style::profile_page>
                        <div class=style::main_content>
                            <div class=style::profile_banner></div>
                            <div class=style::user_info_card>
                                <div class=style::avatar>
                                    <img src=avatar_url.get() onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                </div>
                                <h1>
                                    {format!(
                                        "{} {}",
                                        profile.user.first_name,
                                        profile.user.last_name
                                    )}
                                </h1>
                                <p class=style::status>
                                    {profile.status.clone().unwrap_or_default()}
                                </p>
                            </div>
                            <div class=style::section_card>
                                <h2>"О себе"</h2>
                                <div>{profile.about.clone().unwrap_or_default()}</div>
                            </div>
                        </div>
                        <aside class=style::sidebar>
                            <div class=style::friends_card>
                                <div class=style::friends_header>
                                    <h2>"Друзья"</h2>
                                    <span>{profile.friends_count}</span>
                                </div>
                                <div class=style::friends_grid>
                                    <For
                                        each=move || profile.friends.clone()
                                        key=|friend| friend.user_id
                                        children=move |friend| view! { <FriendCard friend=friend/> }
                                    />
                                </div>
                            </div>
                            <Show when=move || is_current_user>
                                <div class=style::actions_card>
                                    <button on:click=move |_| set_show_modal.set(true)>"Редактировать профиль"</button>
                                    <button class=style::secondary_button>"Настройки"</button>
                                </div>
                            </Show>
                        </aside>

                        <Show when=move || show_modal.get()>
                            <div class=style::modal_backdrop on:click=move |_| set_show_modal.set(false)>
                                <div class=style::modal_content on:click=|e| e.stop_propagation()>
                                    <h2>"Редактировать профиль"</h2>
                                    <form on:submit=on_submit>
                                        <div class=style::avatar_upload_section>
                                            <div class=style::avatar_preview>
                                                <img src=move || preview_url.get().unwrap_or_else(|| avatar_url.get()) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
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
                                            <button type="button" class=style::modal_cancel_button on:click=move |_| set_show_modal.set(false)>"Отмена"</button>
                                        </div>
                                    </form>
                                </div>
                            </div>
                        </Show>
                    </div>
                }.into_any()
            }
            Err(e) => {
                view! { <p class=style::error_message>{e.to_string()}</p> }.into_any()
            }
        })
    };

    view! {
        <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner /></div> }>
            {profile_view}
        </Suspense>
    }
}
