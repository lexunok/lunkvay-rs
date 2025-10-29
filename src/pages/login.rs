use crate::api;
use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::utils::local_storage;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginPage() -> impl IntoView {
    let (is_register_mode, set_is_register_mode) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    // --- Unified Signals ---
    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let email = RwSignal::new("ryan.gosling@gmail.com".to_string());
    let password = RwSignal::new("realhero".to_string());

    let navigate = use_navigate();

    // --- Actions ---
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

    // --- Effects to handle action results ---
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
                    set_is_register_mode.set(false); // Switch to login on success
                }
                Err(e) => {
                    set_error.set(Some(format!("Ошибка регистрации: {}", e)));
                }
            }
        }
    });

    // --- Event Handlers ---
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
        set_error.set(None);
        password.set(String::new());
        set_is_register_mode.set(true);
    };

    let to_login = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        set_error.set(None);
        password.set(String::new());
        first_name.set(String::new());
        last_name.set(String::new());
        set_is_register_mode.set(false);
    };

    view! {
        <div
            class="login-page-container"
            class:login-view=move || !is_register_mode.get()
            class:register-view=move || is_register_mode.get()
            style="width: 100%; height: 100%; position: relative; overflow: hidden;"
        >
            // Login Title
            <div class="title login-title">
                <h1>
                    "Вход в"
                    <br/>
                    "систему"
                </h1>
            </div>

            // Register Title (The Shared Element)
            <div class="title register-title-shared">
                <h1 on:click=to_register style="cursor: pointer;">"Регистрация"</h1>
            </div>

            // Login Form
            <div class="form login-form">
                <form on:submit=on_login_submit style:display="flex" style:flex-direction="column" style:gap="20px">
                    <h2 style="text-align: center; font-size: 36px; font-weight: bold; margin:0 0 20px 0;">"Вход"</h2>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Email"</label>
                        <input type="email" prop:value=email on:input=move |ev| email.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Пароль"</label>
                        <input type="password" prop:value=password on:input=move |ev| password.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <button type="submit" style="background-color: #E2DDBD; color: #1A1A1A; border: none; font-size: 16px; font-weight: bold; padding: 15px 5px; width: 100%; cursor: pointer; border-radius: 5px; margin-top: 10px;">
                        "Войти"
                    </button>
                    <p on:click=to_register style="font-size: 14px; text-align: center; cursor: pointer; margin: 20px 0 0 0;">
                        "Нет аккаунта? Зарегистрироваться"
                    </p>
                </form>
            </div>

            // Register Form
            <div class="form register-form">
                <form on:submit=on_register_submit style:display="flex" style:flex-direction="column" style:gap="15px">
                    <h2 style="text-align: center; font-size: 36px; font-weight: bold; margin:0 0 10px 0;">"Регистрация"</h2>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Имя"</label>
                        <input type="text" prop:value=first_name on:input=move |ev| first_name.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Фамилия"</label>
                        <input type="text" prop:value=last_name on:input=move |ev| last_name.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Email"</label>
                        <input type="email" prop:value=email on:input=move |ev| email.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <div>
                        <label style="font-size: 14px; margin-bottom: 5px; display: block;">"Пароль"</label>
                        <input type="password" prop:value=password on:input=move |ev| password.set(event_target_value(&ev)) style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box; height: 40px;"/>
                    </div>
                    <button type="submit" style="background-color: #E2DDBD; color: #1A1A1A; border: none; font-size: 16px; font-weight: bold; padding: 15px 5px; width: 100%; cursor: pointer; border-radius: 5px; margin-top: 10px;">
                        "Зарегистрироваться"
                    </button>
                    <p on:click=to_login style="font-size: 14px; text-align: center; cursor: pointer; margin: 15px 0 0 0;">
                        "Уже есть аккаунт? Войти"
                    </p>
                </form>
            </div>

            // Error Message Area
            <Show when=move || error.get().is_some()>
                <p style="color: #FF6B6B; font-size: 14px; text-align: center; position: absolute; bottom: 20px; width: 100%; left: 0;">
                    {error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
