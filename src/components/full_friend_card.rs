use crate::{app::BASE_PATH, config::API_BASE_URL, models::user::UserListItem};
use leptos::{ev, prelude::*};
use stylance::import_style;

import_style!(style, "full_friend_card.module.scss");

#[component]
pub fn FullFriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);

    let on_message_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
    };

    let full_name = format!("{} {}", friend.first_name.clone().unwrap_or_default(), friend.last_name.clone().unwrap_or_default());

    view! {
        <div class=style::card>
            <a href=format!("{}/profile/{}", BASE_PATH, friend.user_id) class=style::profile_link>
                <div class=style::avatar>
                    <img src=avatar_url onerror="src='/images/userdefault.webp'"/>
                    <Show when=move || friend.is_online.unwrap_or(false)>
                        <div class=style::online_indicator></div>
                    </Show>
                </div>
                <div class=style::user_info>
                    <span class=style::full_name>{full_name}</span>
                    <span
                        class=style::status
                        data-online=move || friend.is_online.unwrap_or(false).to_string()
                    >
                        {if friend.is_online.unwrap_or(false) { "Online" } else { "Offline" }}
                    </span>
                </div>
            </a>
            <button class=style::message_button on:click=on_message_click>
                "Написать"
            </button>
        </div>
    }
}