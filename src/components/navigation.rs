use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav
            style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                display:flex;
                align-items:center;
                gap:2rem;
                padding:1rem 2rem;
                background-color:#303030;
                color:#E2DDBD;
                font-size:24px;
                box-sizing: border-box;
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
                    text-decoration: none;
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
                    text-decoration: none;
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
                    text-decoration: none;
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
                    text-decoration: none;
                "
            >
                "Чаты"
            </a>

            <a
                href="#"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                    text-decoration: none;
                "
            >
                "Новости"
            </a>

            <a
                href="#"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                    text-decoration: none;
                "
            >
                "Сообщества"
            </a>

            <a
                href="#"
                style="
                    background:none;
                    border:none;
                    color:#E2DDBD;
                    cursor:pointer;
                    font-size:24px;
                    text-decoration: none;
                "
            >
                "Музыка"
            </a>
        </nav>
    }
}
