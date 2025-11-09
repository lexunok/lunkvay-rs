use crate::{
    api::{
        chat_messages::{CreateChatMessageRequest, create_chat_message},
        friends::{
            CreateFriendshipLabelRequest, UpdateFriendshipStatusRequest, create_friendship_label,
            delete_friendship_label, update_friendship_status,
        },
    },
    models::friends::{Friendship, FriendshipStatus},
    utils::API_BASE_URL,
};
use leptos::prelude::*;
use leptos_router::components::A;
use stylance::import_style;
use uuid::Uuid;
use web_sys::SubmitEvent;

import_style!(style, "full_friend_card.module.scss");

#[component]
pub fn FullFriendCard(friend: Friendship, refetch_friends: Callback<()>) -> impl IntoView {
    let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend.user_id);

    let full_name = format!("{} {}", friend.first_name.clone(), friend.last_name.clone());
    let is_online = move || friend.is_online;

    let label = RwSignal::new(String::new());
    let show_message_input = RwSignal::new(false);
    let message_input = RwSignal::new(String::new());

    let create_label_action = Action::new_local(|(friendship_id, label): &(Uuid, String)| {
        let (friendship_id, label) = (friendship_id.clone(), label.clone());

        async move {
            create_friendship_label(CreateFriendshipLabelRequest {
                friendship_id,
                label,
            })
            .await
        }
    });

    let delete_label_action = Action::new_local(|label_id: &Uuid| {
        let label_id = *label_id;
        async move { delete_friendship_label(label_id).await }
    });

    let delete_friendship_action = Action::new_local(|friendship_id: &uuid::Uuid| {
        let friendship_id = *friendship_id;
        async move {
            update_friendship_status(
                friendship_id,
                UpdateFriendshipStatusRequest {
                    status: FriendshipStatus::Deleted,
                },
            )
            .await
        }
    });

    let send_message_action = Action::new_local(move |msg: &String| {
        let msg = msg.clone();
        let receiver_id = friend.user_id;
        async move {
            create_chat_message(CreateChatMessageRequest {
                chat_id: None,
                message: msg,
                receiver_id: Some(receiver_id),
            })
            .await
        }
    });

    Effect::new(move |_| {
        if create_label_action.value().get().is_some() {
            label.set("".to_string());
            refetch_friends.run(());
        }
    });

    Effect::new(move |_| {
        if delete_label_action.version().get() > 0 {
            refetch_friends.run(());
        }
    });

    Effect::new(move |_| {
        if delete_friendship_action.version().get() > 0 {
            refetch_friends.run(());
        }
    });

    Effect::new(move |_| {
        if send_message_action.version().get() > 0 {
            message_input.set(String::new());
            show_message_input.set(false);
        }
    });

    let on_message_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let msg = message_input.get_untracked();
        if !msg.is_empty() {
            send_message_action.dispatch(msg);
        }
    };

    view! {
        <div class=style::card>
            <div class=style::card_header_content>
                <A href=format!("../profile/{}", friend.user_id) attr:class=style::profile_link>
                    <div class=style::avatar>
                        <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                        <Show when=is_online>
                            <div class=style::online_indicator></div>
                        </Show>
                    </div>
                    <div class=style::user_info>
                        <span class=style::full_name>{full_name}</span>
                    </div>
                </A>
                <div>
                    <button class=style::delete_friend_button on:click=move |_| {
                        delete_friendship_action.dispatch(friend.friendship_id.clone());
                    }>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M14 14.252V22H4C4 17.5817 7.58172 14 12 14C12.6906 14 13.3608 14.0875 14 14.252ZM12 13C8.685 13 6 10.315 6 7C6 3.685 8.685 1 12 1C15.315 1 18 3.685 18 7C18 10.315 15.315 13 12 13ZM23 18V20H15V18H23Z"></path></svg>
                    </button>
                </div>
                <div class=style::labels_container>
                    {move || friend.labels.clone().unwrap_or_default().into_iter().map(|label| {
                        let label_id = label.id;
                        view! {
                            <span class=style::label>
                                {label.label}
                                <button class=style::delete_label_button on:click=move |_| { delete_label_action.dispatch(label_id); }>
                                    "x"
                                </button>
                            </span>
                        }
                    }).collect_view()}
                </div>
                <div class=style::label_input_container>
                    <input
                        type="text"
                        placeholder="Добавить метку"
                        class=style::label_input
                        bind:value=label
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                let current_label = label.get_untracked();
                                if !current_label.is_empty() {
                                    create_label_action.dispatch((friend.friendship_id, current_label));
                                }
                            }
                        }
                    />
                </div>
                <button class=style::message_button on:click=move |_| show_message_input.set(!show_message_input.get())>
                    "Написать"
                </button>
            </div>
            <Show when= move || show_message_input.get()>
                <form on:submit=on_message_submit class=style::message_input_area>
                    <input
                        type="text"
                        placeholder="Напишите сообщение..."
                        bind:value=message_input
                    />
                    <button type="submit" class=style::send_message_button>
                        "Отправить"
                    </button>
                </form>
            </Show>
        </div>
    }
}
