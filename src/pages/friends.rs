use crate::api::friends::*;
use crate::models::friends::FriendshipStatus;
use crate::{
    api,
    components::{full_friend_card::FullFriendCard, spinner::Spinner},
    utils::{API_BASE_URL, get_current_user_id},
};
use leptos::prelude::*;
use leptos_router::components::A;
use stylance::import_style;

import_style!(style, "friends.module.scss");

#[component]
pub fn FriendsPage() -> impl IntoView {
    let friends_resource = LocalResource::new(async move || {
        let mut friends = Vec::new();

        if let Some(id) = get_current_user_id() {
            friends = api::friends::get_user_friends(id, None, None)
                .await
                .unwrap_or_default();
        }
        friends
    });

    let incoming_requests_resource = LocalResource::new(async move || {
        get_incoming_friend_requests(None, None)
            .await
            .unwrap_or_default()
    });

    let outgoing_requests_resource = LocalResource::new(async move || {
        get_outgoing_friend_requests(None, None)
            .await
            .unwrap_or_default()
    });

    let possible_friends_resource = LocalResource::new(async move || {
        get_possible_friends(None, None).await.unwrap_or_default()
    });

    let accept_request_action = Action::new_local(|friendship_id: &uuid::Uuid| {
        let friendship_id = *friendship_id;
        async move {
            update_friendship_status(
                friendship_id,
                UpdateFriendshipStatusRequest {
                    status: FriendshipStatus::Accepted,
                },
            )
            .await
        }
    });

    let reject_request_action = Action::new_local(|friendship_id: &uuid::Uuid| {
        let friendship_id = *friendship_id;
        async move {
            update_friendship_status(
                friendship_id,
                UpdateFriendshipStatusRequest {
                    status: FriendshipStatus::Rejected,
                },
            )
            .await
        }
    });

    let cancel_request_action = Action::new_local(|friendship_id: &uuid::Uuid| {
        let friendship_id = *friendship_id;
        async move {
            update_friendship_status(
                friendship_id,
                UpdateFriendshipStatusRequest {
                    status: FriendshipStatus::Cancelled,
                },
            )
            .await
        }
    });

    let send_request_action = Action::new_local(|friendship_id: &uuid::Uuid| {
        let friendship_id = *friendship_id;
        async move { send_friend_request(friendship_id).await }
    });

    Effect::new(move |_| {
        if accept_request_action.version().get() > 0 {
            incoming_requests_resource.refetch();
            friends_resource.refetch();
        }
    });

    Effect::new(move |_| {
        if reject_request_action.version().get() > 0 {
            incoming_requests_resource.refetch();
            possible_friends_resource.refetch();
        }
    });

    Effect::new(move |_| {
        if cancel_request_action.version().get() > 0 {
            outgoing_requests_resource.refetch();
            possible_friends_resource.refetch();
        }
    });

    Effect::new(move |_| {
        if send_request_action.version().get() > 0 {
            outgoing_requests_resource.refetch();
            possible_friends_resource.refetch();
        }
    });

    let friends_view = move || {
        friends_resource.get().map(|friends| {
            view! {
                <div class=style::friends_list>
                    <For
                        each=move || friends.clone()
                        key=|friend| friend.user_id
                        children=move |friend| {
                            view! { <FullFriendCard friend=friend/> }
                        }
                    />
                </div>
            }
            .into_any()
        })
    };

    view! {
        <div class=style::friends_page>
            <div class=style::main_content>
                <div class=style::header>
                    <h1 class=style::title>"Мои друзья"</h1>
                </div>

                <div class=style::search_bar>
                    <input type="text" placeholder="Поиск друзей..." class=style::search_input/>
                    <button class=style::search_button>"Поиск"</button>
                </div>

                <Suspense fallback=|| view! { <div class=style::spinner_container><Spinner/></div> }>
                    {friends_view}
                </Suspense>
            </div>

            <aside class=style::sidebar>
                <div class=style::sidebar_card>
                    <div class=style::sidebar_header>
                        <h2>"Запросы в друзья"</h2>
                    </div>
                    <Suspense fallback=|| view! { <p>"Загрузка запросов..."</p> }>
                        {move || incoming_requests_resource.get().map(|requests| {
                            if requests.is_empty() {
                                view! { <p class=style::placeholder_text>"Нет новых запросов"</p> }.into_any()
                            } else {
                                view! {
                                    <div class=style::friend_requests_list>
                                        <For
                                            each=move || requests.clone()
                                            key=|req| req.friendship_id
                                            children=move |req| {
                                                let sender_name = format!("{} {}", req.first_name, req.last_name);
                                                let friendship_id = req.friendship_id;
                                                let avatar_url = format!("{}/avatar/{}", API_BASE_URL, req.user_id);
                                                let is_online = move || req.is_online;
                                                view! {
                                                    <div class=style::friend_request_item>
                                                        <A href=format!("../profile/{}", req.user_id) attr:class=style::user_info_compact>
                                                            <div class=style::avatar>
                                                                <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                                                <Show when=is_online>
                                                                    <div class=style::online_indicator></div>
                                                                </Show>
                                                            </div>
                                                            <span class=style::sender_name>{sender_name}</span>
                                                        </A>
                                                        <div class=style::request_actions>
                                                            <button class=style::accept on:click=move |_| { accept_request_action.dispatch(friendship_id); }>
                                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM11 11H7V13H11V17H13V13H17V11H13V7H11V11Z"></path></svg>
                                                            </button>
                                                            <button class=style::decline on:click=move |_| { reject_request_action.dispatch(friendship_id); }>
                                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM12 10.5858L9.17157 7.75736L7.75736 9.17157L10.5858 12L7.75736 14.8284L9.17157 16.2426L12 13.4142L14.8284 16.2426L16.2426 14.8284L13.4142 12L16.2426 9.17157L14.8284 7.75736L12 10.5858Z"></path></svg>
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>

                <div class=style::sidebar_card>
                    <div class=style::sidebar_header>
                        <h2>"Исходящие запросы"</h2>
                    </div>
                    <Suspense fallback=|| view! { <p>"Загрузка запросов..."</p> }>
                        {move || outgoing_requests_resource.get().map(|requests| {
                            if requests.is_empty() {
                                view! { <p class=style::placeholder_text>"Нет исходящих запросов"</p> }.into_any()
                            } else {
                                view! {
                                    <div class=style::friend_requests_list>
                                        <For
                                            each=move || requests.clone()
                                            key=|req| req.friendship_id
                                            children=move |req| {
                                                let recipient_name = format!("{} {}", req.first_name, req.last_name);
                                                let friendship_id = req.friendship_id;
                                                let avatar_url = format!("{}/avatar/{}", API_BASE_URL, req.user_id);
                                                let is_online = move || req.is_online;
                                                view! {
                                                    <div class=style::friend_request_item>
                                                        <A href=format!("../profile/{}", req.user_id) attr:class=style::user_info_compact>
                                                            <div class=style::avatar>
                                                                <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                                                <Show when=is_online>
                                                                    <div class=style::online_indicator></div>
                                                                </Show>
                                                            </div>
                                                            <span class=style::sender_name>{recipient_name}</span>
                                                        </A>
                                                        <div class=style::request_actions>
                                                            <button class=style::decline on:click=move |_| { cancel_request_action.dispatch(friendship_id); }>
                                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM12 10.5858L9.17157 7.75736L7.75736 9.17157L10.5858 12L7.75736 14.8284L9.17157 16.2426L12 13.4142L14.8284 16.2426L16.2426 14.8284L13.4142 12L16.2426 9.17157L14.8284 7.75736L12 10.5858Z"></path></svg>
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>

                <div class=style::sidebar_card>
                    <div class=style::sidebar_header>
                        <h2>"Рекомендации"</h2>
                    </div>
                    <Suspense fallback=|| view! { <p>"Загрузка рекомендаций..."</p> }>
                        {move || possible_friends_resource.get().map(|requests| {
                            if requests.is_empty() {
                                view! { <p class=style::placeholder_text>"Нет рекомендаций"</p> }.into_any()
                            } else {
                                view! {
                                    <div class=style::friend_requests_list>
                                        <For
                                            each=move || requests.clone()
                                            key=|req| req.user_id
                                            children=move |req| {
                                                let recipient_name = format!("{} {}", req.first_name, req.last_name);
                                                let friendship_id = req.user_id;
                                                let avatar_url = format!("{}/avatar/{}", API_BASE_URL, req.user_id);
                                                let is_online = move || req.is_online;
                                                view! {
                                                    <div class=style::friend_request_item>
                                                        <A href=format!("../profile/{}", req.user_id) attr:class=style::user_info_compact>
                                                            <div class=style::avatar>
                                                                <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                                                <Show when=is_online>
                                                                    <div class=style::online_indicator></div>
                                                                </Show>
                                                            </div>
                                                            <span class=style::sender_name>{recipient_name}</span>
                                                        </A>
                                                        <div class=style::request_actions>
                                                            <button class=style::accept on:click=move |_| { send_request_action.dispatch(friendship_id); }>
                                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM11 11H7V13H11V17H13V13H17V11H13V7H11V11Z"></path></svg>
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </aside>
        </div>
    }
}
