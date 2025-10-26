use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav
            style="
                display:flex;
                align-items:center;
                gap:2rem;
                padding:1rem 2rem;
                background-color:#303030;
                color:#E2DDBD;
                font-size:24px;
            "
        >
            <a
                href="/profile"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:48px;
                "
            >
                "Lunkvay"
            </a>

            <a
                href="/profile"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                "
            >
                "Профиль"
            </a>

            <a
                href="/friends"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                "
            >
                "Друзья"
            </a>

            <a
                href="/chats"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                "
            >
                "Чаты"
            </a>
        </nav>
    }
}
