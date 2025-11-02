use crate::{models::user::UserListItem, utils::API_BASE_URL};
use leptos::prelude::*;
use leptos_router::components::A;
use stylance::import_style;
import_style!(style, "full_friend_card.module.scss");

#[component]
pub fn FullFriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);

    let full_name = format!(
        "{} {}",
        friend.first_name.clone().unwrap_or_default(),
        friend.last_name.clone().unwrap_or_default()
    );
    let is_online = move || friend.is_online.unwrap_or(false);

    view! {
        <div class=style::card>
            <A href=format!("../profile/{}", friend.user_id) attr:class=style::profile_link>
                <div class=style::avatar>
                    <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                    <Show when=is_online>
                        <div class=style::online_indicator></div>
                    </Show>
                </div>
                <div class=style::user_info>
                    <span class=style::full_name>{full_name}</span>
                    //TODO мб показывать статус или другую информацию
                </div>
            </A>
            <button class=style::message_button>
                "Написать" //TODO
            </button>
        </div>
    }
}
