use crate::components::navigation::Navigation;
use crate::pages::{
    chats::ChatsPage, friends::FriendsPage, login::LoginPage, profile::ProfilePage,
};
use crate::utils::{clear_token, local_storage};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::path;
use stylance::import_style;

import_style!(style, "app.module.scss");

pub const BASE_PATH: &str = if cfg!(debug_assertions) {
    ""
} else {
    "/lunkvay-rs"
};

#[component]
fn MainLayout() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let navigation_is_active = move || !location.pathname.get().ends_with("/auth");

    Effect::new(move |_| {
        let pathname = location.pathname.get();
        let token_is_present = local_storage()
            .and_then(|storage| storage.get_item("token").ok().flatten())
            .is_some();

        if token_is_present {
            if pathname.ends_with("/auth") {
                navigate("/profile", Default::default());
            } else if pathname.ends_with("/logout") {
                clear_token();
                navigate("/auth", Default::default());
            }
        } else {
            if !pathname.ends_with("/auth") {
                navigate("/auth", Default::default());
            }
        }
    });

    view! {
        <div class=style::main_window>
            <Show when=navigation_is_active>
                <Navigation/>
            </Show>

            <main
                class={move || {
                    let conditional_class = if navigation_is_active() {
                        style::with_nav
                    } else {
                        style::without_nav
                    };
                    format!("content {}", conditional_class)
                }}
            >
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
        <Router base=BASE_PATH>
            <MainLayout />
        </Router>
    }
}
