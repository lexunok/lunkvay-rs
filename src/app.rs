use crate::components::navigation::Navigation;
use crate::pages::{
    chats::ChatsPage, friends::FriendsPage, login::LoginPage, profile::ProfilePage,
};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;

#[component]
fn MainLayout() -> impl IntoView {
    let location = use_location();
    let navigation_is_active = move || !location.pathname.get().starts_with("/auth");

    let main_style = move || {
        if navigation_is_active() {
            "
                margin: 100px auto 0 auto;
                width: 100%;
                max-width: 1440px;
                height: calc(100vh - 100px);
                color: white;
                font-size: 24px;
            "
        } else {
            "
                width: 100%;
                height: 100%;
                color: white;
                font-size: 24px;
            "
        }
    };

    view! {
        <div
            class="main-window"
            style="background-color:#1A1A1A; width:100%; height:100%;"
        >
            <Show when=navigation_is_active>
                <Navigation/>
            </Show>

            <main class="content" style=main_style>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/auth") view=LoginPage/>
                    <Route path=path!("/chats") view=ChatsPage/>
                    <Route path=path!("/profile") view=ProfilePage/>
                    <Route path=path!("/friends") view=FriendsPage/>
                </Routes>
            </main>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <MainLayout />
        </Router>
    }
}
