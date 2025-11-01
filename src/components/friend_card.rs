use crate::{config::API_BASE_URL, models::user::UserListItem};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use stylance::import_style;

import_style!(style, "friend_card.module.scss");

#[component]
pub fn FriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);
    let navigate = use_navigate();

    let on_card_click = move |_| {
        let path = format!("/profile/{}", friend.user_id);
        navigate(&path, Default::default());
    };

    view! {
        <div class=style::friend_card on:click=on_card_click>
            <div class=style::friend_avatar>
                <img src=avatar_url onerror="src='/public/images/userdefault.jpg'"/>
                <Show when=move || friend.is_online.unwrap_or(false)>
                    <div class=style::online_indicator></div>
                </Show>
            </div>
            <span>
                {friend.first_name.unwrap_or_default()}
            </span>
        </div>
    }
}
