use crate::{
    api,
    components::{full_friend_card::FullFriendCard, spinner::Spinner},
    utils::get_current_user_id,
};
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "friends.module.scss");

#[component]
pub fn FriendsPage() -> impl IntoView {
    let friends_resource = LocalResource::new(async move || {
        let mut friends = Vec::new();

        if let Some(id) = get_current_user_id() {
            friends = api::friends::get_user_friends(id, None, None)
                .await
                .unwrap_or_default();
        }
        friends
    });

    let friends_view = move || {
        friends_resource.get().map(|friends| {
            view! {
                <div class=style::friends_list>
                    <For
                        each=move || friends.clone()
                        key=|friend| friend.user_id
                        children=move |friend| {
                            view! { <FullFriendCard friend=friend/> }
                        }
                    />
                </div>
            }
            .into_any()
        })
    };

    view! {
        <div class=style::friends_page>
            <div class=style::main_content>
                <div class=style::header>
                    <h1 class=style::title>"Мои друзья"</h1>
                </div>

                <div class=style::search_bar>
                    <input type="text" placeholder="Поиск друзей..." class=style::search_input/>
                    <button class=style::search_button>"Поиск"</button>
                </div>

                <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner/></div> }>
                    {friends_view}
                </Suspense>
            </div>

            <aside class=style::sidebar>
                <div class=style::sidebar_card>
                    <div class=style::sidebar_header>
                        <h2>"Запросы в друзья"</h2>
                    </div>
                    // TODO: Implement friend requests
                    <p class=style::placeholder_text>"Нет новых запросов"</p>
                </div>

                <div class=style::sidebar_card>
                    <div class=style::sidebar_header>
                        <h2>"Рекомендации"</h2>
                    </div>
                    // TODO: Implement recommendations
                    <p class=style::placeholder_text>"Нет рекомендаций"</p>
                </div>
            </aside>
        </div>
    }
}
