use crate::config::API_BASE_URL;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reqwasm::http::{Request, Method};
use serde::Serialize;
use web_sys::Storage;

// --- Structs for API requests ---

#[derive(Serialize, Clone)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize, Clone)]
struct RegisterRequest<'a> {
    #[serde(rename = "firstName")]
    first_name: &'a str,
    #[serde(rename = "lastName")]
    last_name: &'a str,
    email: &'a str,
    password: &'a str,
}

fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

#[component]
pub fn LoginPage() -> impl IntoView {
    // --- State Signals ---
    //let (is_register_mode, set_is_register_mode) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    // --- Input Signals ---
    let email = RwSignal::new(String::from("ryan.gosling@gmail.com"));
    let password=  RwSignal::new(String::from("realhero"));
    let first_name = RwSignal::new(String::new());
    let last_name= RwSignal::new(String::new());

    // --- Actions for API calls ---
    let navigate = use_navigate();

    // let login_action = Action::new(move |(email, password): (String, String)| {
    //     async move {
    //         let request_body = serde_json::to_string(&LoginRequest {
    //             email: &email,
    //             password: &password,
    //         })
    //         .unwrap();

    //         let response = Request::new(&format!("{}/auth/login", API_BASE_URL))
    //             .method(Method::POST)
    //             .header("Content-Type", "application/json")
    //             .body(request_body)
    //             .send()
    //             .await;

    //         match response {
    //             Ok(res) if res.ok() => {
    //                 let token = res.text().await.unwrap_or_default();
    //                 if let Some(storage) = local_storage() {
    //                     _ = storage.set_item("token", &token);
    //                 }
    //                 set_error.set(None);
    //                 navigate("/profile", Default::default());
    //             }
    //             Ok(res) => {
    //                 let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
    //                 set_error.set(Some(format!("Login failed: {}", error_text)));
    //             }
    //             Err(err) => {
    //                 set_error.set(Some(format!("Network error: {}", err)));
    //             }
    //         }
    //     }
    // });

    // let register_action = Action::new(
    //     move |(first_name, last_name, email, password): (String, String, String, String)| {
    //         async move {
    //             let request_body = serde_json::to_string(&RegisterRequest {
    //                 first_name: &first_name,
    //                 last_name: &last_name,
    //                 email: &email,
    //                 password: &password,
    //             })
    //             .unwrap();

    //             let response = Request::new(&format!("{}/auth/register", API_BASE_URL))
    //                 .method(Method::POST)
    //                 .header("Content-Type", "application/json")
    //                 .body(request_body)
    //                 .send()
    //                 .await;

    //             match response {
    //                 Ok(res) if res.ok() => {
    //                     set_error.set(None);
    //                     set_is_register_mode.set(false); // Switch to login form on success
    //                 }
    //                 Ok(res) => {
    //                     let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
    //                     set_error.set(Some(format!("Registration failed: {}", error_text)));
    //                 }
    //                 Err(err) => {
    //                     set_error.set(Some(format!("Network error: {}", err)));
    //                 }
    //             }
    //         }
    //     },
    // );

    // --- Event Handlers ---
    // let on_login_submit = move |ev: ev::SubmitEvent| {
    //     ev.prevent_default();
    //     if login_email().is_empty() || login_password().is_empty() {
    //         set_error.set(Some("Email and password cannot be empty.".to_string()));
    //         return;
    //     }
    //     login_action.dispatch((login_email(), login_password()));
    // };

    // let on_register_submit = move |ev: ev::SubmitEvent| {
    //     ev.prevent_default();
    //     if register_first_name().is_empty()
    //         || register_last_name().is_empty()
    //         || register_email().is_empty()
    //         || register_password().is_empty()
    //     {
    //         set_error.set(Some("All fields are required.".to_string()));
    //         return;
    //     }
    //     register_action.dispatch((
    //         register_first_name(),
    //         register_last_name(),
    //         register_email(),
    //         register_password(),
    //     ));
    // };

    // --- UI ---
    view! {
        <div style="display: flex; justify-content: center; align-items: center; width: 100%; height: 100%; background-color: #1A1A1A;">
            <form on:submit=on_login_submit style="display: flex; flex-direction: column; gap: 20px; width: 300px;">
                <h2 style="color: #E2DDBD; text-align: center; font-size: 36px; font-weight: bold;">"Вход"</h2>

                <div>
                    <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Email"</label>
                    <input
                        type="email"
                        bind:value=email
                        style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
                    />
                </div>

                <div>
                    <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Пароль"</label>
                    <input
                        type="password"
                        bind:value=password
                        style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
                    />
                </div>

                <button type="submit" style="background-color: #E2DDBD; color: #1A1A1A; border: none; font-size: 16px; font-weight: bold; padding: 15px 5px; width: 100%; cursor: pointer; border-radius: 5px;">
                    "Войти"
                </button>

                <a href="/register" style="color: #E2DDBD; font-size: 14px; text-align: center; cursor: pointer; text-decoration: none;">
                    "Нет аккаунта? Зарегистрироваться"
                </a>

                // <Show when=move || error().is_some()>
                //     <p style="color: #FF6B6B; font-size: 14px; text-align: center; max-width: 280px; margin: 0 auto;">
                //         {error}
                //     </p>
                // </Show>
            </form>
        </div>
    }
}


// <form on:submit=on_register_submit style="display: flex; flex-direction: column; gap: 20px; width: 300px;">
//                     <h2 style="color: #E2DDBD; text-align: center; font-size: 36px; font-weight: bold;">"Регистрация"</h2>

//                      <div>
//                         <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Имя"</label>
//                         <input
//                             type="text"
//                             bind:value=first_name
//                             style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
//                         />
//                     </div>

//                      <div>
//                         <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Фамилия"</label>
//                         <input
//                             type="text"
//                             bind:value=last_name
//                             style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
//                         />
//                     </div>

//                      <div>
//                         <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Email"</label>
//                         <input
//                             type="email"
//                             bind:value=email
//                             style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
//                         />
//                     </div>

//                      <div>
//                         <label style="color: #E2DDBD; font-size: 14px; margin-bottom: 5px; display: block;">"Пароль"</label>
//                         <input
//                             type="password"
//                             bind:value=password
//                             style="background-color: #2A2A2A; color: #E2DDBD; border: none; font-size: 16px; padding: 10px; width: 100%; border-radius: 5px; box-sizing: border-box;"
//                         />
//                     </div>

//                     <button type="submit" style="background-color: #E2DDBD; color: #1A1A1A; border: none; font-size: 16px; font-weight: bold; padding: 15px 5px; width: 100%; cursor: pointer; border-radius: 5px;">
//                         "Зарегистрироваться"
//                     </button>

//                     // <p on:click=move |_| set_is_register_mode.set(false) style="color: #E2DDBD; font-size: 14px; text-align: center; cursor: pointer;">
//                     //     "Уже есть аккаунт? Войти"
//                     // </p>

//                     // <Show when=move || error().is_some()>
//                     //     <p style="color: #FF6B6B; font-size: 14px; text-align: center; max-width: 280px; margin: 0 auto;">
//                     //         {error}
//                     //     </p>
//                     // </Show>
//                 </form>