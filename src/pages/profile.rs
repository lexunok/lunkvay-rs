use crate::{
    api::profile::{get_current_user_profile, get_user_profile},
    components::{
        friend_card::FriendCard, profile::editing_window::EditingWindow, spinner::Spinner,
    },
    utils::{API_BASE_URL, get_current_user_id},
};
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "profile.module.scss");

#[derive(Params, PartialEq, Clone, Debug)]
struct ProfileParams {
    id: Option<Uuid>,
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    //SIGNALS
    let params = use_params::<ProfileParams>();
    let (show_editing_window, set_show_editing_window) = signal(false);
    let (avatar_count, set_avatar_count) = signal(0);

    //RESOURCES
    let profile_res = LocalResource::new(move || {
        let params = params.get();
        async move {
            let params = params?;
            if let Some(user_id) = params.id {
                get_user_profile(user_id).await
            } else {
                get_current_user_profile().await
            }
        }
    });

    //VIEW
    view! {
        <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner /></div> }>
            {move || profile_res.get().map(|result| {
                result.map(|profile| {
                    view! {
                        <div class=style::profile_page>
                            <div class=style::main_content>
                                <div class=style::profile_banner></div>
                                <div class=style::user_info_card>
                                    <div class=style::avatar>
                                        <img
                                            src=format!("{}/avatar/{}?v={}", API_BASE_URL, profile.user.id, avatar_count.get())
                                            onerror="this.onerror=null;this.src='/images/userdefault.webp';"
                                        />
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
                                <Show when= move || get_current_user_id().map_or(false, |id| id == profile.user.id)>
                                    <div class=style::actions_card>
                                        <button on:click=move |_| set_show_editing_window.set(true)>"Редактировать профиль"</button>
                                        <button class=style::secondary_button>"Настройки"</button>
                                    </div>
                                </Show>
                            </aside>

                            <Show when=move || show_editing_window.get()>
                                <EditingWindow
                                    status=profile.status.clone().unwrap_or_default()
                                    about=profile.about.clone().unwrap_or_default()
                                    avatar_url = format!("{}/avatar/{}", API_BASE_URL, profile.user.id)
                                    set_show_editing_window = set_show_editing_window
                                    set_avatar_count = set_avatar_count
                                    refetch_profile = Callback::new(move |()| profile_res.refetch())
                                />
                            </Show>
                        </div>
                    }.into_any()
                }).unwrap_or_else(|e| view! { <p class=style::error_message>{e.to_string()}</p> }.into_any())
            })}
        </Suspense>
    }
}
