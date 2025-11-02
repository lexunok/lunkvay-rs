use crate::api;
use crate::models::chat::{ChatListItem, ChatMessage, ChatType, SystemMessageType};
use crate::models::user::User;
use crate::utils::get_current_user_id;
use chrono::Utc;
use leptos::prelude::*;
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "chats.module.scss");

#[component]
pub fn ChatsPage() -> impl IntoView {
    let chats =
        LocalResource::new(async move || api::chat::get_chat_list().await.unwrap_or_default());

    let selected_chat_name = "Alice".to_string();
    let selected_chat_avatar = "https://i.pravatar.cc/150?u=a042581f4e29026024d".to_string();

    let current_user_id = get_current_user_id();

    let messages = vec![
        ChatMessage {
            id: Uuid::new_v4(),
            chat_id: Uuid::new_v4(),
            sender_id: Uuid::new_v4(),
            sender: Some(User {
                id: Uuid::new_v4(),
                user_name: "alice".to_string(),
                email: "alice@example.com".to_string(),
                first_name: Some("Alice".to_string()),
                last_name: None,
                is_online: Some(true),
            }),
            message: "Hey, how is it going?".to_string(),
            created_at: Utc::now(),
            system_message_type: SystemMessageType::None,
        },
        ChatMessage {
            id: Uuid::new_v4(),
            chat_id: Uuid::new_v4(),
            sender_id: Uuid::new_v4(),
            sender: Some(User {
                id: Uuid::new_v4(),
                user_name: "me".to_string(),
                email: "me@example.com".to_string(),
                first_name: Some("Me".to_string()),
                last_name: None,
                is_online: Some(true),
            }),
            message: "Hi Alice! I'm doing great, thanks for asking. How about you?".to_string(),
            created_at: Utc::now(),
            system_message_type: SystemMessageType::None,
        },
    ];

    let (selected_chat_id, set_selected_chat_id) = signal(None::<Uuid>);

    view! {
        <div class=style::container>
            <div class=style::left_panel>
                <div class=style::header>
                    <h1 class=style::title>"Чаты"</h1>
                </div>
                <div class=style::search_bar>
                    // <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    //     <path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z"></path>
                    // </svg>
                    <input type="text" placeholder="Поиск..."/>
                </div>
                <div class=style::chat_list>
                    <For
                        each=move || chats.get().unwrap_or_default()
                        key=|chat| chat.id
                        let(chat)
                    >
                        <div
                            class=style::chat_list_item
                            class:active=move || selected_chat_id.get() == Some(chat.id)
                            on:click=move |_| set_selected_chat_id.set(Some(chat.id))
                        >
                            <img class=style::avatar src={chat.avatar_url.clone().unwrap_or_default()}/>
                            <div class=style::chat_info>
                                <span class=style::chat_name>{chat.name.clone()}</span>
                                <span class=style::last_message>{chat.last_message.clone().unwrap_or_default()}</span>
                            </div>
                            <div class=style::chat_meta>
                                <span>
                                    {chat.last_message_at.map(|t| t.format("%H:%M").to_string()).unwrap_or_default()}
                                </span>
                                {if chat.unread_count > 0 {
                                    view! { <span class=style::unread_count>{chat.unread_count}</span> }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </div>
                        </div>
                    </For>
                </div>
            </div>

            <div class=style::right_panel>
                <div class=style::chat_header>
                    <img class=style::avatar src=selected_chat_avatar.clone()/>
                    <span class=style::chat_name>{selected_chat_name.clone()}</span>
                </div>

                <div class=style::messages_area>
                    <For
                        each=move || messages.clone()
                        key=|msg| msg.id
                        children=move |msg| {
                            let is_my_message = msg.sender_id == current_user_id.unwrap_or_default();
                            let message_class = if is_my_message {
                                style::my_message.to_string()
                            } else {
                                style::other_message.to_string()
                            };

                            match msg.system_message_type {
                                SystemMessageType::None => view! {
                                    <div class=message_class>
                                        <div class=style::message_content>
                                            <p>{msg.message.clone()}</p>
                                            <div class=style::time_and_status>
                                                <span>{msg.created_at.format("%H:%M").to_string()}</span>
                                                {if is_my_message {
                                                    view! {
                                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                                            <path d="M11.602 13.7599L13.014 15.1719L21.4795 6.7063L22.8938 8.12051L13.014 18.0003L6.65 11.6363L8.06421 10.2221L10.189 12.3469L11.6025 13.7594L11.602 13.7599ZM11.6037 10.9322L16.5563 5.97949L17.9666 7.38977L13.014 12.3424L11.6037 10.9322ZM8.77698 16.5873L7.36396 18.0003L1 11.6363L2.41421 10.2221L3.82723 11.6352L3.82604 11.6363L8.77698 16.5873Z"></path>
                                                        </svg>
                                                    }.into_any()
                                                } else {
                                                    view! { <span/> }.into_any()
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }.into_any(),
                                _ => view! {
                                    <div class=style::system_message>
                                        <p>{msg.message.clone()}</p>
                                    </div>
                                }.into_any()
                            }
                        }
                    />
                </div>

                <div class=style::message_input_area>
                    <button class=style::icon_button>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M14 13.5V8C14 5.79086 12.2091 4 10 4C7.79086 4 6 5.79086 6 8V13.5C6 17.0899 8.91015 20 12.5 20C16.0899 20 19 17.0899 19 13.5V4H21V13.5C21 18.1944 17.1944 22 12.5 22C7.80558 22 4 18.1944 4 13.5V8C4 4.68629 6.68629 2 10 2C13.3137 2 16 4.68629 16 8V13.5C16 15.433 14.433 17 12.5 17C10.567 17 9 15.433 9 13.5V8H11V13.5C11 14.3284 11.6716 15 12.5 15C13.3284 15 14 14.3284 14 13.5Z"></path>
                        </svg>
                    </button>
                    <input type="text" placeholder="Напишите сообщение..."/>
                    <button class=format!("{} {}", style::icon_button, style::send_button)>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M3 12.9999H9V10.9999H3V1.84558C3 1.56944 3.22386 1.34558 3.5 1.34558C3.58425 1.34558 3.66714 1.36687 3.74096 1.40747L22.2034 11.5618C22.4454 11.6949 22.5337 11.9989 22.4006 12.2409C22.3549 12.324 22.2865 12.3924 22.2034 12.4381L3.74096 22.5924C3.499 22.7255 3.19497 22.6372 3.06189 22.3953C3.02129 22.3214 3 22.2386 3 22.1543V12.9999Z"></path>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    }
}
