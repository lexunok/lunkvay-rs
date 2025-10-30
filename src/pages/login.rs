use crate::api;
use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::utils::local_storage;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use stylance::import_style;

import_style!(style, "login.module.scss");


#[component]
pub fn LoginPage() -> impl IntoView {

    // Сигналы
    let (is_register_mode, set_is_register_mode) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let email = RwSignal::new("ryan.gosling@gmail.com".to_string());
    let password = RwSignal::new("realhero".to_string());

    let navigate = use_navigate();

    // Экшоны для асинхронного запроса
    let login_action = Action::new_local(|(email, password): &(String, String)| {
        let email = email.clone();
        let password = password.clone();
        async move {
            let creds = LoginRequest {
                email: &email,
                password: &password,
            };
            api::auth::login(creds).await.map_err(|e| e.to_string())
        }
    });

    let register_action = Action::new_local(
        |(first_name, last_name, email, password): &(String, String, String, String)| {
            let first_name = first_name.clone();
            let last_name = last_name.clone();
            let email = email.clone();
            let password = password.clone();
            async move {
                let details = RegisterRequest {
                    first_name: &first_name,
                    last_name: &last_name,
                    email: &email,
                    password: &password,
                };
                api::auth::register(details)
                    .await
                    .map_err(|e| e.to_string())
            }
        },
    );

    // Эффекты для обработки результата с экшона
    Effect::new(move |_| {
        if let Some(result) = login_action.value().get() {
            match result {
                Ok(token) => {
                    if let Some(storage) = local_storage() {
                        let _ = storage.set_item("token", &token);
                    }
                    set_error.set(None);
                    navigate("/profile", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(format!("Ошибка входа: {}", e)));
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(result) = register_action.value().get() {
            match result {
                Ok(_) => {
                    set_error.set(None);
                    set_is_register_mode.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Ошибка регистрации: {}", e)));
                }
            }
        }
    });

    // Ивенты
    let on_login_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some("Почта или пароль не могут быть пустыми.".to_string()));
            return;
        }
        login_action.dispatch_local((email.get(), password.get()));
    };

    let on_register_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if first_name.get().is_empty()
            || last_name.get().is_empty()
            || email.get().is_empty()
            || password.get().is_empty()
        {
            set_error.set(Some("Все поля обязательны для заполнения.".to_string()));
            return;
        }
        register_action.dispatch_local((
            first_name.get(),
            last_name.get(),
            email.get(),
            password.get(),
        ));
    };

    let to_register = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_is_register_mode.set(true);
    };

    let to_login = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_is_register_mode.set(false);
    };

    view! {
        <div
            class=style::login_page_container
            class:login_view=move || !is_register_mode.get()
            class:register_view=move || is_register_mode.get()
        >
            <div class=format!("{} {}", style::title, style::login_title)>
                <h1>
                    "Вход в"
                    <br/>
                    "систему"
                </h1>
            </div>

            <div class=format!("{} {}", style::title, style::register_title_shared)>
                <h1 on:click=to_register class=style::register_title_h1>"Регистрация"</h1>
            </div>

            <div class=format!("{} {}", style::form_wrapper, style::login_form_wrapper)>
                <form on:submit=on_login_submit class=style::login_form>
                    <h2>"Вход"</h2>
                    <div>
                        <label>"Email"</label>
                        <input type="email" bind:value=email/>
                    </div>
                    <div>
                        <label>"Пароль"</label>
                        <input type="password" bind:value=password/>
                    </div>
                    <button type="submit">
                        "Войти"
                    </button>
                    <p on:click=to_register>
                        "Нет аккаунта? Зарегистрироваться"
                    </p>
                </form>
            </div>

            <div class=format!("{} {}", style::form_wrapper, style::register_form_wrapper)>
                <form on:submit=on_register_submit class=style::register_form>
                    <h2>"Регистрация"</h2>
                    <div>
                        <label>"Имя"</label>
                        <input type="text" bind:value=first_name/>
                    </div>
                    <div>
                        <label>"Фамилия"</label>
                        <input type="text" bind:value=last_name/>
                    </div>
                    <div>
                        <label>"Email"</label>
                        <input type="email" bind:value=email/>
                    </div>
                    <div>
                        <label>"Пароль"</label>
                        <input type="password" bind:value=password/>
                    </div>
                    <button type="submit">
                        "Зарегистрироваться"
                    </button>
                    <p on:click=to_login>
                        "Уже есть аккаунт? Войти"
                    </p>
                </form>
            </div>

            <Show when=move || error.get().is_some()>
                <p class=style::error_message>
                    {error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
