use crate::config::API_BASE_URL;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reqwasm::http::{Method, Request};
use serde::{Deserialize, Serialize};
use web_sys::Storage;

#[derive(Serialize)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct RegisterRequest<'a> {
    #[serde(rename = "firstName")]
    first_name: &'a str,
    #[serde(rename = "lastName")]
    last_name: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
}

fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

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
            let request_body = serde_json::to_string(&LoginRequest { email: &email, password: &password }).unwrap();
            let request = Request::new(&format!("{}/auth/login", API_BASE_URL))
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(request_body);

            match request.send().await {
                Ok(response) if response.ok() => {
                    let token = response.text().await.unwrap_or_default();
                    Ok(token)
                }
                Ok(response) => {
                    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(error_text)
                }
                Err(e) => Err(e.to_string()),
            }
        }
    });

    let register_action = Action::new_local(|(first_name, last_name, email, password): &(String, String, String, String)| {
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let email = email.clone();
        let password = password.clone();
        async move {
            let request_body = serde_json::to_string(&RegisterRequest {
                first_name: &first_name,
                last_name: &last_name,
                email: &email,
                password: &password,
            }).unwrap();

            let request = Request::new(&format!("{}/auth/register", API_BASE_URL))
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(request_body);

            match request.send().await {
                Ok(response) if response.ok() => Ok(()),
                Ok(response) => {
                    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(error_text)
                }
                Err(e) => Err(e.to_string()),
            }
        }
    });

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
        if first_name.get().is_empty() || last_name.get().is_empty() || email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some("Все поля обязательны для заполнения.".to_string()));
            return;
        }
        register_action.dispatch_local((first_name.get(), last_name.get(), email.get(), password.get()));
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
        <div style="width: 100%; height: 100%; display: flex; overflow: hidden;">
            <div
                style:width="300%"
                style:height="100%"
                style:display="flex"
                style:transform=move || {
                    if is_register_mode.get() {
                        "translateX(-66.66%)"
                    } else {
                        "translateX(0)"
                    }
                }
                style:transition="transform 0.3s cubic-bezier(0.25, 0.1, 0.25, 1.0)"
            >
                <div style="width: 33.33%; height: 100%; display: flex; align-items: center; justify-content: center; padding: 50px; box-sizing: border-box;">
                    <h1 style="font-size: 5vw; font-weight: bold; margin: 0;">"Вход в систему"</h1>
                </div>

                <div style="width: 33.33%; height: 100%; display: flex; align-items: center; justify-content: center; position: relative;">
                    <div style:width="300px" style:height="400px" style:position="relative">
                        <form
                            on:submit=on_login_submit
                            style:display="flex"
                            style:flex-direction="column"
                            style:gap="20px"
                            style:width="100%"
                            style:position="absolute"
                            style:opacity=move || if is_register_mode.get() { "0" } else { "1" }
                            style:transition="opacity 0.2s ease-in-out"
                            style:pointer-events=move || if is_register_mode.get() { "none" } else { "auto" }
                        >
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

                        <form
                            on:submit=on_register_submit
                            style:display="flex"
                            style:flex-direction="column"
                            style:gap="15px"
                            style:width="100%"
                            style:position="absolute"
                            style:opacity=move || if is_register_mode.get() { "1" } else { "0" }
                            style:transition="opacity 0.2s ease-in-out"
                            style:pointer-events=move || if is_register_mode.get() { "auto" } else { "none" }
                        >
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
                    <Show when=move || error.get().is_some()>
                        <p style="color: #FF6B6B; font-size: 14px; text-align: center; max-width: 280px; position: absolute; bottom: 20px; left: 0; right: 0; margin: auto;">
                            {error}
                        </p>
                    </Show>
                </div>

                <div style="width: 33.33%; height: 100%; display: flex; align-items: center; justify-content: center; padding: 50px; box-sizing: border-box;">
                     <h1 style="font-size: 5vw; font-weight: bold; margin: 0;">"Регистрация"</h1>
                </div>
            </div>
        </div>
    }
}
