use crate::{config::API_BASE_URL, models::user::UserListItem};
use leptos::{ev, prelude::*};
use stylance::import_style;

import_style!(style, "full_friend_card.module.scss");

#[component]
pub fn FullFriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);

    let on_message_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        // TODO: Navigate to chat with this user
    };

    let full_name = format!("{} {}", friend.first_name.clone().unwrap_or_default(), friend.last_name.clone().unwrap_or_default());

    view! {
        <a href=format!{"/profile/{}", friend.user_id} class=style::card>
            <div class=style::left>
                <div class=style::avatar>
                    <img src=avatar_url onerror="src='/images/userdefault.jpg'"/>
                    <Show when=move || friend.is_online.unwrap_or(false)>
                        <div class=style::online_indicator></div>
                    </Show>
                </div>
                <div class=style::user_info>
                    <span class=style::full_name>{full_name}</span>
                    <span class=style::status>"Offline"</span>
                </div>
            </div>
            <div class=style::right>
                <button class=style::message_button on:click=on_message_click>
                    "Написать"
                </button>
            </div>
        </a>
    }
}
