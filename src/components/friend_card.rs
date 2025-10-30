use crate::{config::API_BASE_URL, models::user::UserListItem};
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "friend_card.module.scss");

#[component]
pub fn FriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);

    view! {
        <div class=style::friend_card>
            <div class=style::friend_avatar>
                <img src=avatar_url alt="friend avatar"/>
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
