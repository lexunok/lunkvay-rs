use crate::components::spinner::Spinner;
use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::utils::local_storage;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use stylance::import_style;
use crate::api::auth::{register, login};

import_style!(style, "login.module.scss");

#[component]
pub fn LoginPage() -> impl IntoView {
    //SIGNALS
    let is_register_mode = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let user_name = RwSignal::new(String::new());
    let email = RwSignal::new("ryan.gosling@gmail.com".to_string());
    let password = RwSignal::new("realhero".to_string());

    let navigate = use_navigate();

    //ACTIONS
    let login_action = Action::new_local(|(email, password): &(String, String)| {
        let (email, password) = (email.clone(), password.clone());
        async move {
            login(LoginRequest { email, password })
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
                register(RegisterRequest {
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

    //EFFECTS
    Effect::new(move |_| {
        if let Some(result) = login_action.value().get() {
            match result {
                Ok(token) => {
                    if let Some(storage) = local_storage() {
                        let _ = storage.set_item("token", &token);
                    }
                    error.set(None);
                    navigate("/profile", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(result) = register_action.value().get() {
            match result {
                Ok(_) => {
                    error.set(None);
                    is_register_mode.set(false);
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    //EVENTS
    let on_login = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let email = email.get_untracked();
        let password = password.get_untracked();
        if email.is_empty() || password.is_empty() {
            error.set(Some("Почта или пароль не могут быть пустыми.".to_string()));
            return;
        }
        login_action.dispatch_local((email, password));
    };

    let on_register = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let first_name = first_name.get_untracked();
        let last_name = last_name.get_untracked();
        let user_name = user_name.get_untracked();
        let email = email.get_untracked();
        let password = password.get_untracked();

        if first_name.is_empty() || last_name.is_empty() || user_name.is_empty() || email.is_empty() || password.is_empty()  {
            error.set(Some("Все поля обязательны для заполнения.".to_string()));
            return;
        }
        register_action.dispatch_local((
            first_name,
            last_name,
            user_name,
            email,
            password,
        ));
    };

    let change_form = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        is_register_mode.set(!is_register_mode.get_untracked());
    };

    let is_loading = move || login_action.pending().get() || register_action.pending().get();

    //VIEW
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
                <h1 on:click=change_form class=style::register_title_h1>"Регистрация"</h1>
            </div>

            <div class=format!("{} {}", style::form_wrapper, style::login_form_wrapper)>
                <form on:submit=on_login class=style::login_form>
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
                    <p on:click=change_form>
                        "Нет аккаунта? Зарегистрироваться"
                    </p>
                </form>
            </div>

            <div class=format!("{} {}", style::form_wrapper, style::register_form_wrapper)>
                <form on:submit=on_register class=style::register_form>
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
                    <p on:click=change_form>
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
