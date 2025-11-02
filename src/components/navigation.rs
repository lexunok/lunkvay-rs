use leptos::prelude::*;
use leptos_router::components::A;
use stylance::import_style;
import_style!(style, "navigation.module.scss");

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class=style::nav_container>
            <div class=style::nav_left>
                <A href="./profile" attr:class=style::logo_link>
                    "Lunkvay"
                </A>
                <A href="./profile" attr:class=style::nav_link>
                    "Профиль"
                </A>
                <A href="./friends" attr:class=style::nav_link>
                    "Друзья"
                </A>
                <A href="./chats" attr:class=style::nav_link>
                    "Чаты"
                </A>
            </div>
            <div class=style::nav_right>
                <A href="./logout" attr:class=style::logout_button>
                    "Выйти"
                </A>
            </div>
        </nav>
    }
}
