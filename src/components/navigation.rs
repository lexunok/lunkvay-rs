use leptos::prelude::*;
use stylance::import_style;
import_style!(style, "navigation.module.scss");

#[component]
pub fn Navigation() -> impl IntoView {

    view! {
        <nav class=style::nav_container>
            <a href="/profile" class=style::logo_link>
                "Lunkvay"
            </a>

            <a href="/profile" class=style::nav_link>
                "Профиль"
            </a>

            <a href="/friends" class=style::nav_link>
                "Друзья"
            </a>

            <a href="/chats" class=style::nav_link>
                "Чаты"
            </a>

            <a href="/logout" class=style::nav_link>
                "Выйти"
            </a>
        </nav>
    }
}
