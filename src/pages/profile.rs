use crate::{
    api,
    components::{friend_card::FriendCard, spinner::Spinner},
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
    let params = use_params::<ProfileParams>();

    let profile_resource = LocalResource::new(move || {
        let params_result = params.get();
        async move {
            let params = params_result?;
            if let Some(user_id) = params.id {
                api::profile::get_user_profile(user_id).await
            } else {
                api::profile::get_current_user_profile().await
            }
        }
    });

    let profile_view = move || {
        profile_resource.get().map(|result| match result {
            Ok(profile) => {
                let avatar_url = format!(
                    "{}/avatar/{}",
                    API_BASE_URL,
                    profile.user.id
                );
                let is_current_user = get_current_user_id().map_or(false, |id| id == profile.user.id);

                view! {
                    <div class=style::profile_page>
                        <div class=style::main_content>
                            <div class=style::profile_banner></div>
                            <div class=style::user_info_card>
                                <div class=style::avatar>
                                    <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                </div>
                                <h1>
                                    {format!(
                                        "{} {}",
                                        profile.user.first_name,
                                        profile.user.last_name
                                    )}
                                </h1>
                                <p class=style::status>
                                    {profile.status.unwrap_or_default()}
                                </p>
                            </div>
                            <div class=style::section_card>
                                <h2>"О себе"</h2>
                                <div>{profile.about.unwrap_or_default()}</div>
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
                                        let(friend)
                                    >
                                        <FriendCard friend=friend/>
                                    </For>
                                </div>
                            </div>
                            <Show when=move ||is_current_user>
                                <div class=style::actions_card>
                                    <button>"Редактировать профиль"</button>
                                    <button class=style::secondary_button>"Настройки"</button>
                                </div>
                            </Show>
                        </aside>
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
