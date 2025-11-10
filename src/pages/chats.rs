use crate::api::chat::get_all_chats;
use crate::components::chat::create_chat_window::CreateChatWindow;
use crate::components::chat::messages::Messages;
use crate::components::spinner::Spinner;
use crate::models::chat::Chat;
use crate::utils::{API_BASE_URL, get_current_user_id};
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "chats.module.scss");

#[component]
pub fn ChatsPage() -> impl IntoView {
    //RESOURCES
    let chats = LocalResource::new(async move || get_all_chats().await.unwrap_or_default());
    //SIGNALS
    let (selected_chat, set_chat) = signal(None::<Chat>);
    let (show_create_chat_window, set_show_create_chat_window) = signal(false);
    let avatar_count = RwSignal::new(0);
    //VIEW
    view! {
        <div class=style::container>
            <div class=style::left_panel>
                <div class=style::header>
                    <h1 class=style::title>"Чаты"</h1>
                    <button class=style::create_chat_button on:click=move |_| set_show_create_chat_window.set(true)>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11h-4v4h-2v-4H7v-2h4V7h2v4h4v2z"></path>
                        </svg>
                    </button>
                </div>
                <div class=style::search_bar>
                    <svg class=style::search_icon xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z"></path>
                    </svg>
                    <input type="text" placeholder="Поиск..."/>
                </div>
                <div class=style::chat_list>
                    <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner/></div> }>
                        <For
                            each=move || chats.get().unwrap_or_default()
                            key=|chat| chat.id
                            children=move |chat| {
                                let chat_image = format!(
                                    "{}/chat-image/{}/{}?v={}",
                                    API_BASE_URL,
                                    get_current_user_id().unwrap_or_default(),
                                    chat.id,
                                    avatar_count.get()
                                );
                                view! {
                                    <div
                                        class=style::chat_list_item
                                        style=move || {
                                            if selected_chat.get().map_or(false, |c| c.id == chat.id) {
                                                "background-color: #3366CC;"
                                            } else {
                                                ""
                                            }
                                        }
                                        on:click=move |_| set_chat.set(Some(chat.clone()))
                                    >
                                        {
                                            match &chat.last_message {
                                                Some(data) => {
                                                    view! {
                                                        <img class=style::avatar src=chat_image.clone() onerror="this.onerror=null;this.src='/images/chatdefault.webp';"/>
                                                        <div class=style::chat_info>
                                                            <span class=style::chat_name>{chat.name.clone().unwrap_or_default()}</span>
                                                            <span class=style::last_message>{data.message.clone()}</span>
                                                        </div>
                                                        <div class=style::chat_meta>
                                                            <span>
                                                                {data.created_at.format("%H:%M").to_string()}
                                                            </span>
                                                        </div>
                                                    }.into_any()
                                                }
                                                None => {
                                                    view!{
                                                        <img class=style::avatar src=chat_image.clone() onerror="this.onerror=null;this.src='/images/chatdefault.webp';"/>
                                                        <div class=style::chat_info>
                                                            <span class=style::chat_name>{chat.name.clone().unwrap_or_default()}</span>
                                                        </div>
                                                        <div class=style::chat_meta>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }
                                        }
                                    </div>
                                }.into_any()
                            }
                        />
                    </Suspense>
                </div>
            </div>

            <div class=style::right_panel>
                {move || selected_chat.get().map(|chat| {
                    view! {
                        <Messages chat = chat set_chat=set_chat avatar_count=avatar_count refetch_chats=Callback::new(move |()| chats.refetch())/>
                    }.into_any()
                }).unwrap_or_else(|| view! {<div class=style::no_chat_selected><h1>"Выберите чат чтобы начать общение"</h1></div>}.into_any())
                }
            </div>

            <Show when=move || show_create_chat_window.get()>
                <CreateChatWindow
                    set_show_create_chat_window=set_show_create_chat_window
                    refetch_chats=Callback::new(move |()| chats.refetch())
                />
            </Show>
        </div>
    }
}
