use crate::api;
use crate::components::spinner::Spinner;
use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::utils::local_storage;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use stylance::import_style;

import_style!(style, "login.module.scss");

#[component]
pub fn LoginPage() -> impl IntoView {
    let (is_register_mode, set_is_register_mode) = signal_local(false);
    let (error, set_error) = signal_local(None::<String>);

    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let user_name = RwSignal::new(String::new());
    let email = RwSignal::new("ryan.gosling@gmail.com".to_string());
    let password = RwSignal::new("realhero".to_string());

    let navigate = use_navigate();

    let login_action = Action::new_local(|(email, password): &(String, String)| {
        let (email, password) = (email.clone(), password.clone());
        async move {
            api::auth::login(LoginRequest { email, password })
                .await
                .map_err(|e| e.to_string())
        }
    });

    let register_action = Action::new_local(
        |(first, last, user, email, pass): &(String, String, String, String, String)| {
            let (first_name, last_name, user_name, email, password) = (
                first.clone(),
                last.clone(),
                user.clone(),
                email.clone(),
                pass.clone(),
            );
            async move {
                api::auth::register(RegisterRequest {
                    first_name,
                    last_name,
                    user_name,
                    email,
                    password,
                })
                .await
                .map_err(|e| e.to_string())
            }
        },
    );

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
                    set_error.set(Some(e));
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
                    set_error.set(Some(e));
                }
            }
        }
    });

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
        if [first_name, last_name, user_name, email, password]
            .iter()
            .any(|v| v.get().is_empty())
        {
            set_error.set(Some("Все поля обязательны для заполнения.".to_string()));
            return;
        }
        register_action.dispatch_local((
            first_name.get(),
            last_name.get(),
            user_name.get(),
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

    let is_loading = move || login_action.pending().get() || register_action.pending().get();

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
                    <div class=style::loading_overlay style:display=move || if is_loading() { "flex" } else { "none" } >
                        <Spinner />
                    </div>
                    <h2>"Вход"</h2>
                    <div>
                        <label>"Email"</label>
                        <input type="email" prop:disabled=is_loading bind:value=email/>
                    </div>
                    <div>
                        <label>"Пароль"</label>
                        <input type="password" prop:disabled=is_loading bind:value=password/>
                    </div>
                    <button type="submit" prop:disabled=is_loading>
                        "Войти"
                    </button>
                    <p on:click=to_register>
                        "Нет аккаунта? Зарегистрироваться"
                    </p>
                </form>
            </div>

            <div class=format!("{} {}", style::form_wrapper, style::register_form_wrapper)>
                <form on:submit=on_register_submit class=style::register_form>
                    <div class=style::loading_overlay style:display=move || if is_loading() { "flex" } else { "none" } >
                        <Spinner />
                    </div>
                    <h2>"Регистрация"</h2>
                    <div>
                        <label>"Имя"</label>
                        <input type="text" prop:disabled=is_loading bind:value=first_name/>
                    </div>
                    <div>
                        <label>"Фамилия"</label>
                        <input type="text" prop:disabled=is_loading bind:value=last_name/>
                    </div>
                    <div>
                        <label>"Nickname"</label>
                        <input type="text" prop:disabled=is_loading bind:value=user_name/>
                    </div>
                    <div>
                        <label>"Email"</label>
                        <input type="email" prop:disabled=is_loading bind:value=email/>
                    </div>
                    <div>
                        <label>"Пароль"</label>
                        <input type="password" prop:disabled=is_loading bind:value=password/>
                    </div>
                    <button type="submit" prop:disabled=is_loading>
                        "Зарегистрироваться"
                    </button>
                    <p on:click=to_login>
                        "Уже есть аккаунт? Войти"
                    </p>
                </form>
            </div>

            <Show when=move || error.get().is_some() && !is_loading()>
                <p class=style::error_message>
                    {error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
