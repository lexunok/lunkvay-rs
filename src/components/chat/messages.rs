use crate::{
    api::{
        self,
        chat_messages::{
            CreateChatMessageRequest, DeleteChatMessageRequest, UpdateEditChatMessageRequest,
            UpdatePinChatMessageRequest,
        },
    },
    components::{chat::chat_settings_window::ChatSettingsWindow, spinner::Spinner},
    models::chat::{
        Chat, ChatMessage, ChatType, PinnedMessageData, SystemMessageType, WsMessage, WsMessageType,
    },
    utils::{API_BASE_URL, DOMAIN, get_current_user_id},
};
use chrono::{NaiveDate, Utc};
use codee::string::JsonSerdeCodec;
use leptos::html::Div;
use leptos::{ev, prelude::*};
use leptos_use::{
    UseInfiniteScrollOptions, UseWebSocketReturn, use_event_listener,
    use_infinite_scroll_with_options, use_websocket,
};
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "messages.module.scss");

const PAGE_SIZE: u32 = 20;

#[derive(Clone, Debug, PartialEq)]
enum ListItem {
    Message(ChatMessage),
    DateSeparator(NaiveDate),
}

#[derive(Clone, Debug, PartialEq)]
struct ContextMenuState {
    message_id: Uuid,
    is_my_message: bool,
    is_pinned: bool,
    x: i32,
    y: i32,
}

#[component]
pub fn Messages(
    chat: Chat,
    set_chat: WriteSignal<Option<Chat>>,
    refetch_chats: Callback<()>,
) -> impl IntoView {
    let chat_id = chat.id;
    let chat_cloned = chat.clone();

    let UseWebSocketReturn { message, .. } = use_websocket::<(), WsMessage, JsonSerdeCodec>(
        &format!("wss://{}/ws?roomId={}", DOMAIN, chat_id),
    );

    //SIGNALS
    let chat_image = format!("{}/chat-image/{}", API_BASE_URL, chat_id);
    let messsage_input = RwSignal::new(String::new());
    let messages = RwSignal::new(Vec::<ChatMessage>::new());
    let current_page = RwSignal::new(1);
    let has_more_messages = RwSignal::new(true);
    let messages_area_ref = NodeRef::<Div>::new();
    let is_loading_more = RwSignal::new(false);
    let show_pinned = RwSignal::new(false);
    let context_menu_state: RwSignal<Option<ContextMenuState>> = RwSignal::new(None);
    let editing_message_id: RwSignal<Option<Uuid>> = RwSignal::new(None);
    let edit_input = RwSignal::new(String::new());
    let (show_chat_settings_window, set_show_chat_settings_window) = signal(false);

    //RESOURCES
    let initial_messages = LocalResource::new(move || async move {
        api::chat_messages::get_chat_messages(chat_id, None, Some(1), Some(PAGE_SIZE))
            .await
            .unwrap_or_default()
    });

    let pinned_messages = LocalResource::new(move || async move {
        api::chat_messages::get_chat_messages(chat_id, Some(true), Some(1), Some(5))
            .await
            .unwrap_or_default()
    });

    //ACTIONS
    let send_message = Action::new_local(move |input: &CreateChatMessageRequest| {
        let input = input.clone();
        async move { api::chat_messages::create_chat_message(input).await }
    });

    let delete_message_action = Action::new_local(move |message_id: &Uuid| {
        let message_id = *message_id;
        async move {
            let req = DeleteChatMessageRequest {
                chat_id,
                message_id,
            };
            let _ = api::chat_messages::delete_chat_message(req).await;
        }
    });

    let pin_message_action = Action::new_local(move |(message_id, is_pinned): &(Uuid, bool)| {
        let message_id = *message_id;
        let is_pinned = *is_pinned;
        async move {
            let req = UpdatePinChatMessageRequest {
                chat_id,
                message_id,
                is_pinned,
            };
            let _ = api::chat_messages::update_pin_chat_message(req).await;
        }
    });

    let edit_message_action =
        Action::new_local(move |(message_id, new_message): &(Uuid, String)| {
            let message_id = *message_id;
            let new_message = new_message.clone();
            async move {
                let req = UpdateEditChatMessageRequest {
                    chat_id,
                    message_id,
                    new_message,
                };
                let _ = api::chat_messages::update_edit_chat_message(req).await;
            }
        });

    //EFFECTS
    Effect::new(move |_| {
        if let Some(mut initial) = initial_messages.get() {
            if initial.len() < PAGE_SIZE as usize {
                has_more_messages.set(false);
            }
            initial.reverse();
            messages.set(initial);
            if let Some(element) = messages_area_ref.get() {
                request_animation_frame(move || {
                    element.set_scroll_top(element.scroll_height());
                });
            }
        }
    });

    Effect::new(move |_| {
        if let Some(ws_message) = message.get() {
            match ws_message.r#type {
                WsMessageType::ReceiveMessage => {
                    if let Ok(mut chat_message) =
                        serde_json::from_value::<ChatMessage>(ws_message.data)
                    {
                        if chat_message.sender_id == get_current_user_id() {
                            chat_message.is_my_message = true;
                        }
                        if let Some(messages_area) = messages_area_ref.get() {
                            let should_scroll = messages_area.scroll_top()
                                + messages_area.client_height()
                                >= messages_area.scroll_height() - 200;
                            messages.update(|msgs| msgs.push(chat_message));
                            if should_scroll {
                                request_animation_frame(move || {
                                    messages_area.set_scroll_top(messages_area.scroll_height());
                                });
                            }
                        }
                    }
                }
                WsMessageType::MessageUpdated => {
                    if let Ok(updated_message) =
                        serde_json::from_value::<ChatMessage>(ws_message.data)
                    {
                        editing_message_id.set(None);
                        context_menu_state.set(None);
                        messages.update(|msgs| {
                            if let Some(pos) = msgs.iter().position(|m| m.id == updated_message.id)
                            {
                                msgs[pos] = updated_message;
                                *msgs = msgs.clone();
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
                                msg.updated_at = pinned_data.updated_at;
                            }
                        });
                        if show_pinned.get_untracked() {
                            pinned_messages.refetch();
                        }
                    }
                }
                WsMessageType::ChatUpdated => {
                    if let Ok(chat) = serde_json::from_value::<Chat>(ws_message.data) {
                        if let Some(messages_area) = messages_area_ref.get() {
                            if let Some(chat_message) = chat.last_message.clone() {
                                let should_scroll = messages_area.scroll_top()
                                    + messages_area.client_height()
                                    >= messages_area.scroll_height() - 200;
                                messages.update(|msgs| msgs.push(chat_message));
                                if should_scroll {
                                    request_animation_frame(move || {
                                        messages_area.set_scroll_top(messages_area.scroll_height());
                                    });
                                }
                            }
                        }
                        set_chat.set(Some(chat));
                    }
                }
                WsMessageType::ChatDeleted => {
                    refetch_chats.run(());
                    set_chat.set(None);
                }
                _ => {}
            }
        }
    });

    Effect::new(move |_| {
        if show_pinned.get() {
            pinned_messages.refetch();
        }
    });

    Effect::new(move |_| {
        if let Some(element) = messages_area_ref.get() {
            let _ = use_event_listener(element, ev::click, move |_| {
                context_menu_state.set(None);
            });
        }
    });

    let _ = use_infinite_scroll_with_options(
        messages_area_ref,
        move |_| async move {
            if is_loading_more.get_untracked() || !has_more_messages.get_untracked() {
                return;
            }

            is_loading_more.set(true);
            current_page.update(|p| *p += 1);

            if let Some(messages_area) = messages_area_ref.get_untracked() {
                let old_scroll_height = messages_area.scroll_height();

                if let Ok(mut new_messages) = api::chat_messages::get_chat_messages(
                    chat_id,
                    None,
                    Some(current_page.get_untracked()),
                    Some(PAGE_SIZE),
                )
                .await
                {
                    if new_messages.is_empty() || new_messages.len() < PAGE_SIZE as usize {
                        has_more_messages.set(false);
                    }

                    new_messages.reverse();
                    messages.update(|msgs| {
                        let mut new_msgs = new_messages;
                        new_msgs.extend(msgs.clone());
                        *msgs = new_msgs;
                    });

                    request_animation_frame(move || {
                        let new_scroll_height = messages_area.scroll_height();
                        messages_area.set_scroll_top(new_scroll_height - old_scroll_height);
                        is_loading_more.set(false);
                    });
                } else {
                    is_loading_more.set(false);
                }
            } else {
                is_loading_more.set(false);
            }
        },
        UseInfiniteScrollOptions::default().direction(leptos_use::core::Direction::Top),
    );

    let messages_with_dates = Memo::new(move |_| {
        let mut result: Vec<ListItem> = Vec::new();
        let mut last_date: Option<NaiveDate> = None;

        for msg in messages.get() {
            let msg_date = msg.created_at.date();
            if last_date.is_none() || last_date != Some(msg_date) {
                result.push(ListItem::DateSeparator(msg_date));
                last_date = Some(msg_date);
            }
            result.push(ListItem::Message(msg));
        }
        result
    });

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

    let chat_name = chat.name.unwrap_or_default();
    let chat_type_cloned = chat.chat_type.clone();
    view! {
        <div class=style::messages_container>
            <div class=style::chat_header>
                <img class=style::avatar src=chat_image onerror="this.onerror=null;this.src='/images/chatdefault.webp';"/>
                <span class=style::chat_name>{chat_name}</span>
                <Show when=move || chat.chat_type.clone() == ChatType::Group>
                    <button class=style::header_button on:click=move |_| set_show_chat_settings_window.set(true)>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M6.45455 19L2 22.5V4C2 3.44772 2.44772 3 3 3H21C21.5523 3 22 3.44772 22 4V18C22 18.5523 21.5523 19 21 19H6.45455ZM8.14499 12.071L7.16987 12.634L8.16987 14.366L9.1459 13.8025C9.64746 14.3133 10.2851 14.69 11 14.874V16H13V14.874C13.7149 14.69 14.3525 14.3133 14.8541 13.8025L15.8301 14.366L16.8301 12.634L15.855 12.071C15.9495 11.7301 16 11.371 16 11C16 10.629 15.9495 10.2699 15.855 9.92901L16.8301 9.36602L15.8301 7.63397L14.8541 8.19748C14.3525 7.68674 13.7149 7.31003 13 7.12602V6H11V7.12602C10.2851 7.31003 9.64746 7.68674 9.1459 8.19748L8.16987 7.63397L7.16987 9.36602L8.14499 9.92901C8.0505 10.2699 8 10.629 8 11C8 11.371 8.0505 11.7301 8.14499 12.071ZM12 13C10.8954 13 10 12.1046 10 11C10 9.89543 10.8954 9 12 9C13.1046 9 14 9.89543 14 11C14 12.1046 13.1046 13 12 13Z"></path></svg>
                    </button>
                </Show>
                <button class=style::header_button on:click=move |_| show_pinned.update(|v| *v = !*v)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M22.3126 10.1753L20.8984 11.5895L20.1913 10.8824L15.9486 15.125L15.2415 18.6606L13.8273 20.0748L9.58466 15.8321L4.63492 20.7819L3.2207 19.3677L8.17045 14.4179L3.92781 10.1753L5.34202 8.76107L8.87756 8.05396L13.1202 3.81132L12.4131 3.10422L13.8273 1.69L22.3126 10.1753Z"></path></svg>
                </button>
            </div>

            <div
                class=move || {
                    let mut classes = vec![style::pinned_messages_bar.to_string()];
                    if show_pinned.get() {
                        classes.push(style::show.to_string());
                    }
                    classes.join(" ")
                }
            >
                {move || pinned_messages.get().map(|msgs| view! {
                    <For
                        each=move || msgs.clone()
                        key=|msg| msg.id
                        children=|msg| {
                            view! {
                                <div class=style::pinned_message_item>
                                    <strong>{msg.sender_user_name.clone()}</strong>
                                    <p>{msg.message.clone()}</p>
                                    <Show when=move || msg.pinned_at.is_some()>
                                        <span class=style::pinned_at_time>{msg.pinned_at.unwrap().format("%d.%m.%y %H:%M").to_string()}</span>
                                    </Show>
                                </div>
                            }
                        }
                    />
                })}
            </div>

            <div class=style::messages_area node_ref=messages_area_ref>
                <Show when=move || is_loading_more.get()>
                    <div class=style::spinner_container><Spinner/></div>
                </Show>
                <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner/></div> }>
                     <For
                        each=move || messages_with_dates.get()
                        key=|item| match item {
                            ListItem::Message(msg) => (msg.id.to_string(), msg.updated_at.unwrap_or_default().to_string().clone()),
                            ListItem::DateSeparator(date) => (date.to_string(), date.to_string()),
                        }
                        children=move |item| {
                            match item {
                                ListItem::Message(msg) => {
                                    let message_class = if msg.is_my_message {
                                        style::my_message.to_string()
                                    } else {
                                        style::other_message.to_string()
                                    };
                                    let chat_type_cloned_2 = chat_type_cloned.clone();
                                    let created_at = msg.created_at;
                                    let now = Utc::now();
                                    let time_str = if created_at.date() == now.date_naive() {
                                        created_at.format("%H:%M").to_string()
                                    } else {
                                        created_at.format("%d.%m.%y %H:%M").to_string()
                                    };

                                    match msg.system_message_type {
                                        SystemMessageType::None => view! {
                                            <div
                                                class=message_class
                                                on:contextmenu=move |ev| {
                                                    ev.prevent_default();
                                                    if let Some(area) = messages_area_ref.get() {
                                                        let area_rect = area.get_bounding_client_rect();
                                                        let x_offset = -150;
                                                        let y_offset = -60;
                                                        let x = ev.client_x() - area_rect.left() as i32 + area.scroll_left() as i32 + x_offset;
                                                        let y = ev.client_y() - area_rect.top() as i32 + area.scroll_top() as i32 + y_offset;

                                                        context_menu_state.set(Some(ContextMenuState {
                                                            message_id: msg.id,
                                                            is_my_message: msg.is_my_message,
                                                            is_pinned: msg.is_pinned,
                                                            x,
                                                            y,
                                                        }));
                                                    }
                                                }
                                            >
                                                <Show when=move || !msg.is_my_message && chat_type_cloned_2.clone() != ChatType::Personal>
                                                    <div class=style::sender_name>{msg.sender_user_name.clone()}</div>
                                                </Show>

                                                <Show
                                                    when=move || editing_message_id.get() != Some(msg.id)
                                                    fallback=move || {
                                                        let msg_id = msg.id;
                                                        view! {
                                                            <div class=style::edit_container>
                                                                <input
                                                                    type="text"
                                                                    class=style::edit_input
                                                                    bind:value=edit_input
                                                                    on:keyup=move |ev| {
                                                                        if ev.key() == "Enter" {
                                                                            edit_message_action.dispatch((msg_id, edit_input.get()));
                                                                        } else if ev.key() == "Escape" {
                                                                            editing_message_id.set(None);
                                                                        }
                                                                    }
                                                                />
                                                                <div class=style::edit_buttons>
                                                                    <button on:click=move |_| {
                                                                        edit_message_action.dispatch((msg_id, edit_input.get()));
                                                                    }>{"Сохранить"}</button>
                                                                    <button on:click=move |_| editing_message_id.set(None)>{"Отмена"}</button>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    <div class=style::message_content>
                                                        <p>{msg.message.clone()}</p>
                                                    </div>
                                                </Show>

                                                <div class=style::time_and_status>
                                                    <Show when=move || msg.is_pinned>
                                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M22.3126 10.1753L20.8984 11.5895L20.1913 10.8824L15.9486 15.125L15.2415 18.6606L13.8273 20.0748L9.58466 15.8321L4.63492 20.7819L3.2207 19.3677L8.17045 14.4179L3.92781 10.1753L5.34202 8.76107L8.87756 8.05396L13.1202 3.81132L12.4131 3.10422L13.8273 1.69L22.3126 10.1753Z"></path></svg>
                                                    </Show>
                                                    <Show when=move || msg.is_edited>
                                                        <span class=style::edited_indicator>"(изм.)"</span>
                                                    </Show>
                                                    <span>{time_str}</span>
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
                                        }.into_any(),
                                        _ => view! {
                                            <div class=style::system_message>
                                                <p>{msg.message.clone()}</p>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                ListItem::DateSeparator(date) => {
                                    let today = Utc::now().date_naive();
                                    let date_str = if date == today {
                                        "Сегодня".to_string()
                                    } else if date == today.pred_opt().unwrap_or(today) {
                                        "Вчера".to_string()
                                    } else {
                                        date.format("%d %B %Y").to_string()
                                    };
                                    view! {
                                        <div class=style::date_separator>
                                            <span>{date_str}</span>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    />
                </Suspense>
                <Show when=move || context_menu_state.get().is_some()>
                    {move || context_menu_state.get().map(|state| {
                        view! {
                            <div
                                class=style::context_menu
                                style=format!("left: {}px; top: {}px;", state.x, state.y)
                            >
                                <button on:click=move |_| {
                                    pin_message_action.dispatch((state.message_id, !state.is_pinned));
                                    context_menu_state.set(None);
                                }>{if state.is_pinned {"Открепить"} else {"Закрепить"}}</button>
                                <Show when=move || state.is_my_message>
                                    <button on:click=move |_| {
                                        if let Some(msg) = messages.get().iter().find(|m| m.id == state.message_id).cloned() {
                                            edit_input.set(msg.message);
                                            editing_message_id.set(Some(state.message_id));
                                        }
                                        context_menu_state.set(None);
                                    }>{"Редактировать"}</button>
                                    <button on:click=move |_| {
                                        delete_message_action.dispatch(state.message_id);
                                        context_menu_state.set(None);
                                    }>{"Удалить"}</button>
                                </Show>
                            </div>
                        }
                    })}
                </Show>
            </div>

            <div class=style::message_input_area>
                <button class=style::icon_button>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M14 13.5V8C14 5.79086 12.2091 4 10 4C7.79086 4 6 5.79086 6 8V13.5C6 17.0899 8.91015 20 12.5 20C16.0899 20 19 17.0899 19 13.5V4H21V13.5C21 18.1944 17.1944 22 12.5 22C7.80558 22 4 18.1944 4 13.5V8C4 4.68629 6.68629 2 10 2C13.3137 2 16 4.68629 16 8V13.5C16 15.433 14.433 17 12.5 17C10.567 17 9 15.433 9 13.5V8H11V13.5C11 14.3284 11.6716 15 12.5 15C13.3284 15 14 14.3284 14 13.5Z"></path>
                    </svg>
                </button>
                <form on:submit=|ev| ev.prevent_default()>
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
                </form>
            </div>
            <Show when=move || show_chat_settings_window.get()>
                <ChatSettingsWindow
                    chat=chat_cloned.clone()
                    set_show_chat_settings_window=set_show_chat_settings_window
                    refetch_chats=refetch_chats
                />
            </Show>
        </div>
    }
}
