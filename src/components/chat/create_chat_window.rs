use crate::api::{
    chat::{CreateGroupChatRequest, create_group_chat},
    friends::get_friends,
};
use crate::utils::API_BASE_URL;
use leptos::prelude::*;
use stylance::import_style;
use uuid::Uuid;
use web_sys::SubmitEvent;

import_style!(style, "create_chat_window.module.scss");

#[component]
pub fn CreateChatWindow(
    set_show_create_chat_window: WriteSignal<bool>,
    refetch_chats: Callback<()>,
) -> impl IntoView {
    // SIGNALS
    let chat_name = RwSignal::new(String::new());
    let (selected_friends, set_selected_friends) = signal(Vec::<Uuid>::new());

    // RESOURCES
    let friends_res = LocalResource::new(move || get_friends(None, None));

    // ACTIONS
    let create_chat_action = Action::new_local(move |req: &CreateGroupChatRequest| {
        let req = req.clone();
        async move { create_group_chat(req).await }
    });

    // EFFECTS
    Effect::new(move |_| {
        if create_chat_action.version().get() > 0 {
            if create_chat_action.value().get().is_some() {
                refetch_chats.run(());
                set_show_create_chat_window.set(false);
            }
        }
    });

    // EVENT HANDLERS
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let name = chat_name.get_untracked();
        let members = selected_friends.get_untracked();
        if !name.is_empty() && !members.is_empty() {
            create_chat_action.dispatch(CreateGroupChatRequest { name, members });
        }
    };

    let toggle_friend_selection = move |friend_id: Uuid| {
        set_selected_friends.update(move |friends| {
            if friends.contains(&friend_id) {
                friends.retain(|&id| id != friend_id);
            } else {
                friends.push(friend_id);
            }
        });
    };

    view! {
        <div class=style::backdrop on:click=move |_| set_show_create_chat_window.set(false)>
            <div class=style::content on:click=|e| e.stop_propagation()>
                <h2>"Создать новый чат"</h2>
                <form on:submit=on_submit>
                    <div class=style::form_field>
                        <label for="chat_name">"Название чата"</label>
                        <input
                            type="text"
                            id="chat_name"
                            bind:value=chat_name
                            placeholder="Введите название чата"
                        />
                    </div>

                    <div class=style::form_field>
                        <label>"Выберите участников"</label>
                        <div class=style::friends_list>
                            <Suspense fallback=|| view! { <p>"Загрузка друзей..."</p> }>
                                {move || {
                                    friends_res.get().map(|friends_result| {
                                        match friends_result {
                                            Ok(friends) => {
                                                if friends.is_empty() {
                                                    view! { <p>"У вас пока нет друзей."</p> }.into_any()
                                                } else {
                                                    friends.into_iter().map(|friendship| {
                                                        let friend_id = friendship.user_id;
                                                        let is_selected = move || selected_friends.get().contains(&friend_id);
                                                        let avatar_url = format!("{}/avatar/{}", API_BASE_URL, friend_id);
                                                        view! {
                                                            <div
                                                                class=move || format!("{} {}", style::friend_item, if is_selected() { style::selected } else { "" })
                                                                on:click=move |_| toggle_friend_selection(friend_id)
                                                            >
                                                                <img src=avatar_url onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                                                <span>{format!("{} {}", friendship.first_name, friendship.last_name)}</span>
                                                            </div>
                                                        }
                                                    }).collect_view().into_any()
                                                }
                                            },
                                            Err(e) => view! { <p>"Ошибка загрузки друзей: "{e.to_string()}</p> }.into_any(),
                                        }
                                    })
                                }}
                            </Suspense>
                        </div>
                    </div>

                    <div class=style::form_actions>
                        <button type="submit">"Создать"</button>
                        <button type="button" class=style::cancel_button on:click=move |_| set_show_create_chat_window.set(false)>"Отмена"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
