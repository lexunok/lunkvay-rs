use crate::{api, api::error::ApiError, components::friend_card::FriendCard, config::API_BASE_URL, utils::{clear_token, get_current_user_id}};
use leptos::prelude::*;
use leptos_router::{hooks::{use_navigate, use_params}, params::Params};
use stylance::import_style;

import_style!(style, "profile.module.scss");

#[derive(Params, PartialEq, Clone, Debug)]
struct ProfileParams {
    id: Option<String>,
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    let navigate = use_navigate();
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
                            <header class=style::profile_header>
                                <div class=style::avatar>
                                    <img src=avatar_url alt="User avatar" onerror="this.onerror=null;this.src='/public/images/userdefault.jpg';"/>
                                </div>
                                <div class=style::user_info>
                                    <h1>
                                        {format!(
                                            "{} {}",
                                            profile.user.first_name.clone().unwrap_or_default(),
                                            profile.user.last_name.clone().unwrap_or_default()
                                        )}
                                    </h1>
                                    <p class=style::status>
                                        {profile.status.clone().unwrap_or_default()}
                                    </p>
                                </div>
                            </header>
                            <section class=style::about_section>
                                <h2>"Информация"</h2>
                                <div class=style::about_box>
                                    <h3>"О себе"</h3>
                                    <p>{profile.about.clone().unwrap_or_default()}</p>
                                </div>
                            </section>
                        </div>
                        <aside class=style::sidebar>
                            <div class=style::friends_section>
                                <div class=style::friends_header>
                                    <h2>"Друзья"</h2>
                                    <span>{profile.friends_count.unwrap_or(0)}</span>
                                </div>
                                <div class=style::friends_grid>
                                    <For
                                        each=move || profile.friends.clone().unwrap_or_default()
                                        key=|friend| friend.user_id
                                        children=move |friend| {
                                            view! { <FriendCard friend=friend/> }
                                        }
                                    />
                                </div>
                            </div>
                            <Show when=move || is_current_user>
                                <div class=style::actions>
                                    <button>"Редактировать профиль"</button>
                                    <button class=style::secondary_button>"Настройки"</button>
                                </div>
                            </Show>
                        </aside>
                    </div>
                }.into_any()
            }
            Err(e) => {
                if let ApiError::Unauthorized = e {
                    clear_token();
                    navigate("/auth", Default::default());
                    return view! { <div/> }.into_any(); 
                }
                view! { <p class=style::error_message>{format!("Ошибка загрузки профиля: {}", e)}</p> }.into_any()
            }
        })
    };


    view! {
        <Suspense fallback=|| view! { <p>"Загрузка профиля..."</p> }>
            {profile_view}
        </Suspense>
    }
}
