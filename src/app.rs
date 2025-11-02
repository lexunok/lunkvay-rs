use crate::components::navigation::Navigation;
use crate::pages::{
    chats::ChatsPage, friends::FriendsPage, login::LoginPage, profile::ProfilePage,
};
use crate::utils::{clear_token, has_token};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::path;
use stylance::import_style;

import_style!(style, "app.module.scss");

#[component]
fn MainLayout() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let navigation_is_active = move || !location.pathname.get().ends_with("/auth");

    // Следим за изменением пути и токена
    Effect::new(move |_| {
        let pathname = location.pathname.get();
        let token_present = has_token();

        match (token_present, pathname.as_str()) {
            // пользователь вошёл, но находится на /auth → перенаправляем в профиль
            (true, path) if path.ends_with("/auth") => navigate("/profile", Default::default()),
            // пользователь вошёл и перешёл на /logout → очищаем токен и редиректим на /auth
            (true, path) if path.ends_with("/logout") => {
                clear_token();
                navigate("/auth", Default::default());
            }
            // пользователь не вошёл → не пускаем, если он не на /auth
            (false, path) if !path.ends_with("/auth") => navigate("/auth", Default::default()),
            _ => {}
        }
    });

    let main_class = move || {
        if navigation_is_active() {
            style::with_nav
        } else {
            style::without_nav
        }
    };

    view! {
        <div class=style::main_window>
            <Show when=navigation_is_active>
                <Navigation/>
            </Show>

            <main class=main_class>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=|| view! { <div/> }/>
                    <Route path=path!("/auth") view=LoginPage/>
                    <Route path=path!("/chats") view=ChatsPage/>
                    <Route path=path!("/profile") view=ProfilePage/>
                    <Route path=path!("/profile/:id") view=ProfilePage/>
                    <Route path=path!("/friends") view=FriendsPage/>
                </Routes>
            </main>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/lunkvay-rs">
            <MainLayout />
        </Router>
    }
}
