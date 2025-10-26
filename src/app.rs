use crate::components::navigation::Navigation;
use crate::pages::{
    chats::ChatsPage, friends::FriendsPage, login::LoginPage, profile::ProfilePage,
};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    let (navigation_is_active, _set_navigation_is_active) = signal(true);

    view! {
        <Router>
            <div
                class="main-window"
                style="background-color:#1A1A1A; width:100%; height:100%;"
            >
                <Show when=move || navigation_is_active.get()>
                    <Navigation/>
                </Show>

                <main
                    class="content"
                    style="
                            margin-top:100px;
                            width:100%;
                            height:calc(100vh - 100px);
                            color:white;
                            font-size:24px;
                        "
                >
                    <Routes fallback= || "Not found.">
                        <Route path=path!("/auth") view=LoginPage/>
                        <Route path=path!("/chats") view=ChatsPage/>
                        <Route path=path!("/profile") view=ProfilePage/>
                        <Route path=path!("/friends") view=FriendsPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
