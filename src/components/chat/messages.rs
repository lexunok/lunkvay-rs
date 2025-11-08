use crate::{
    api::{self, chat_messages::CreateChatMessageRequest},
    components::spinner::Spinner,
    models::chat::{
        Chat, ChatMessage, PinnedMessageData, SystemMessageType, WsMessage, WsMessageType,
    },
    utils::{API_BASE_URL, DOMAIN, get_current_user_id},
};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::{UseWebSocketReturn, use_websocket};
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "messages.module.scss");

#[component]
pub fn Messages(chat: Chat) -> impl IntoView {
    let chat_id = chat.id;

    let UseWebSocketReturn { message, .. } = use_websocket::<(), WsMessage, JsonSerdeCodec>(
        &format!("wss://{}/ws?roomId={}", DOMAIN, chat_id),
    );

    //SIGNALS
    let chat_image = format!("{}/chat-image/{}", API_BASE_URL, chat_id);
    let messsage_input = RwSignal::new(String::new());
    let messages = RwSignal::new(Vec::<ChatMessage>::new());

    //RESOURCES
    let initial_messages = LocalResource::new(move || async move {
        api::chat_messages::get_chat_messages(chat_id, None, None, None)
            .await
            .unwrap_or_default()
    });

    //EFFECTS
    Effect::new(move |_| {
        if let Some(initial) = initial_messages.get() {
            messages.set(initial);
        }
    });

    Effect::new(move |_| {
        if let Some(ws_message) = message.get() {
            match ws_message.r#type {
                WsMessageType::ReceiveMessage => {
                    if let Ok(mut chat_message) =
                        serde_json::from_value::<ChatMessage>(ws_message.data)
                    {
                        if let Some(id) = get_current_user_id() {
                            chat_message.is_my_message = chat_message.sender_id == id;
                        }
                        messages.update(|msgs| msgs.push(chat_message));
                    }
                }
                WsMessageType::MessageUpdated => {
                    if let Ok(updated_message) =
                        serde_json::from_value::<ChatMessage>(ws_message.data)
                    {
                        messages.update(|msgs| {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == updated_message.id)
                            {
                                *msg = updated_message;
                            }
                        });
                    }
                }
                WsMessageType::MessageDeleted => {
                    if let Ok(message_id) = serde_json::from_value::<Uuid>(ws_message.data) {
                        messages.update(|msgs| msgs.retain(|m| m.id != message_id));
                    }
                }
                WsMessageType::MessagePinned => {
                    if let Ok(pinned_data) =
                        serde_json::from_value::<PinnedMessageData>(ws_message.data)
                    {
                        messages.update(|msgs| {
                            if let Some(msg) =
                                msgs.iter_mut().find(|m| m.id == pinned_data.message_id)
                            {
                                msg.is_pinned = pinned_data.is_pinned;
                            }
                        });
                    }
                }
                _ => {}
            }
        }
    });

    //ACTIONS
    let send_message = Action::new_local(move |input: &CreateChatMessageRequest| {
        let input = input.clone();
        async move { api::chat_messages::create_chat_message(input).await }
    });

    //EVENTS
    let on_submit = move || {
        let msg = messsage_input.get_untracked();
        if !msg.is_empty() {
            let request = CreateChatMessageRequest {
                chat_id: Some(chat_id),
                message: msg,
                receiver_id: None,
            };
            messsage_input.set(String::new());
            send_message.dispatch(request);
        }
    };
    //VIEW
    view! {
        <div class=style::chat_header>
            <img class=style::avatar src=chat_image onerror="this.onerror=null;this.src='/images/chatdefault.webp';"/>
            <span class=style::chat_name>{chat.name.unwrap_or_default()}</span>
        </div>

        <div class=style::messages_area>
            <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner/></div> }>
                 <For
                    each=move || messages.get()
                    key=|msg| msg.id
                    children=move |msg| {
                        let message_class = if msg.is_my_message {
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
                                            {if msg.is_my_message {
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
            </Suspense>
        </div>

        <div class=style::message_input_area>
            <button class=style::icon_button>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M14 13.5V8C14 5.79086 12.2091 4 10 4C7.79086 4 6 5.79086 6 8V13.5C6 17.0899 8.91015 20 12.5 20C16.0899 20 19 17.0899 19 13.5V4H21V13.5C21 18.1944 17.1944 22 12.5 22C7.80558 22 4 18.1944 4 13.5V8C4 4.68629 6.68629 2 10 2C13.3137 2 16 4.68629 16 8V13.5C16 15.433 14.433 17 12.5 17C10.567 17 9 15.433 9 13.5V8H11V13.5C11 14.3284 11.6716 15 12.5 15C13.3284 15 14 14.3284 14 13.5Z"></path>
                </svg>
            </button>
            <input
                type="text"
                placeholder="Напишите сообщение..."
                bind:value=messsage_input
                on:keyup=move |ev| {
                    if ev.key() == "Enter" {
                        on_submit();
                    }
                }
            />
            <button
                class=format!("{} {}", style::icon_button, style::send_button)
                on:click=move |_| on_submit()
            >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M3 12.9999H9V10.9999H3V1.84558C3 1.56944 3.22386 1.34558 3.5 1.34558C3.58425 1.34558 3.66714 1.36687 3.74096 1.40747L22.2034 11.5618C22.4454 11.6949 22.5337 11.9989 22.4006 12.2409C22.3549 12.324 22.2865 12.3924 22.2034 12.4381L3.74096 22.5924C3.499 22.7255 3.19497 22.6372 3.06189 22.3953C3.02129 22.3214 3 22.2386 3 22.1543V12.9999Z"></path>
                </svg>
            </button>
        </div>
    }
}
