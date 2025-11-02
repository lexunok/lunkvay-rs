use crate::{models::user::UserListItem, utils::API_BASE_URL};
use leptos::prelude::*;
use leptos_router::components::A;
use stylance::import_style;
import_style!(style, "friend_card.module.scss");

#[component]
pub fn FriendCard(friend: UserListItem) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);
    let is_online = move || friend.is_online.unwrap_or(false);

    view! {
        <A href=format!("./{}", friend.user_id) attr:class=style::friend_card>
            <div class=style::friend_avatar>
                <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                <Show when=is_online>
                    <div class=style::online_indicator></div>
                </Show>
            </div>
            <span>
                {friend.first_name.unwrap_or_default()}
            </span>
        </A>
    }
}
