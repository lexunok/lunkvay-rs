use crate::app::BASE_PATH;
use leptos::prelude::*;
use stylance::import_style;
import_style!(style, "navigation.module.scss");

#[component]
pub fn Navigation() -> impl IntoView {

    view! {
        <nav class=style::nav_container>
            <div class=style::nav_left>
                <a href=format!("{}/profile", BASE_PATH) class=style::logo_link>
                    "Lunkvay"
                </a>
                <a href=format!("{}/profile", BASE_PATH) class=style::nav_link>
                    "Профиль"
                </a>
                <a href=format!("{}/friends", BASE_PATH) class=style::nav_link>
                    "Друзья"
                </a>
                <a href=format!("{}/chats", BASE_PATH) class=style::nav_link>
                    "Чаты"
                </a>
            </div>
            <div class=style::nav_right>
                <a href=format!("{}/logout", BASE_PATH) class=style::logout_button>
                    "Выйти"
                </a>
            </div>
        </nav>
    }
}
