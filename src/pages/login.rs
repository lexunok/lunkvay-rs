use leptos::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    // состояния (signal)
    let (is_register, set_is_register) = signal(false);
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (register_email, set_register_email) = signal(String::new());
    let (register_password, set_register_password) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (loading, set_loading) = signal(false);

    // обработчик логина (через reqwasm)
    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let set_error = set_error.clone();
        let set_loading = set_loading.clone();
        let on_login = on_login.clone();

        move |ev: web_sys::Event| {
            ev.prevent_default();
            let email = email.get();
            let password = password.get();
            let set_error = set_error.clone();
            let set_loading = set_loading.clone();
            let on_login = on_login.clone();

            spawn_local(async move {
                set_loading.set(true);
                set_error.set(String::new());

                let req = LoginRequest { email, password };

                let resp = Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&req).unwrap())
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status() == 200 => {
                        match r.json::<LoginResponse>().await {
                            Ok(json) => {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.set_item("jwt", &json.token);
                                    }
                                }
                                // если передан WriteSignal<Page>, переключаем страницу
                                if let Some(nav) = on_login {
                                    nav.set(Page::CHATS);
                                }
                            }
                            Err(e) => {
                                set_error.set(format!("Ошибка парсинга ответа: {}", e));
                            }
                        }
                    }
                    Ok(r) => {
                        set_error.set(format!("Ошибка входа: HTTP {}", r.status()));
                    }
                    Err(e) => {
                        set_error.set(format!("Сетевая ошибка: {}", e));
                    }
                }

                set_loading.set(false);
            });
        }
    };

    view! {
        <main class="login-root" style="width:100%;height:100%;display:flex;justify-content:center;align-items:center;background:#1A1A1A;color:#E2DDBD;">
            <form style="width:320px; display:flex; flex-direction:column;">
                {move || if !is_register.get() {
                    view! {
                        <>
                            <h1 style="text-align:center; font-size:24px; margin-bottom:12px;">"Вход в систему"</h1>

                            <label style="font-size:14px; margin-bottom:6px;">"Email"</label>
                            <input
                                prop:value=email
                                on:input=move |e| set_email.set(event_target_value(&e))
                                class="input"
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;"
                                placeholder="Email"
                            />

                            <label style="font-size:14px; margin-bottom:6px;">"Пароль"</label>
                            <input
                                prop:value=password
                                type="password"
                                on:input=move |e| set_password.set(event_target_value(&e))
                                class="input"
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;"
                                placeholder="Пароль"
                            />

                            <button type="submit" disabled=move || loading.get() style="background:#E2DDBD;color:#1A1A1A;border:0;padding:12px;font-weight:bold;margin-top:6px;">
                                {move || if loading.get() { "Вход..." } else { "Войти" }}
                            </button>

                            <p style="text-align:center; margin-top:12px; cursor:pointer; font-size:14px;"
                               on:click=move |_| set_is_register.set(true)>
                                "Нет аккаунта? Зарегистрироваться"
                            </p>

                            <p style="color:#FF6B6B; text-align:center; margin-top:8px;">{move || error.get()}</p>
                        </>
                    }.into_any()
                } else {
                    view! {
                        <>
                            <h1 style="text-align:center; font-size:24px; margin-bottom:12px;">"Регистрация"</h1>

                            <label style="font-size:14px; margin-bottom:6px;">"Имя"</label>
                            <input prop:value=first_name on:input=move |e| set_first_name.set(event_target_value(&e))
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;" />

                            <label style="font-size:14px; margin-bottom:6px;">"Фамилия"</label>
                            <input prop:value=last_name on:input=move |e| set_last_name.set(event_target_value(&e))
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;" />

                            <label style="font-size:14px; margin-bottom:6px;">"Email"</label>
                            <input prop:value=register_email on:input=move |e| set_register_email.set(event_target_value(&e))
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;" />

                            <label style="font-size:14px; margin-bottom:6px;">"Пароль"</label>
                            <input prop:value=register_password type="password" on:input=move |e| set_register_password.set(event_target_value(&e))
                                style="background:#2A2A2A; color:#E2DDBD; border:0; padding:10px; margin-bottom:12px;" />

                            <button type="button" on:click=move |_| {
                                // реализуй регистрацию здесь (аналогично логину)
                            } style="background:#E2DDBD;color:#1A1A1A;border:0;padding:12px;font-weight:bold;margin-top:6px;">
                                "Зарегистрироваться"
                            </button>

                            <p style="text-align:center; margin-top:12px; cursor:pointer; font-size:14px;"
                               on:click=move |_| set_is_register.set(false)>
                                "Уже есть аккаунт? Войти"
                            </p>

                            <p style="color:#FF6B6B; text-align:center; margin-top:8px;">{move || error.get()}</p>
                        </>
                    }.into_any()
                }}
            </form>
        </main>
    }
}
