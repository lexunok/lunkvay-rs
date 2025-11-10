use crate::{
    api::{
        chat_members::{
            CreateChatMemberRequest, DeleteChatMemberRequest, UpdateChatMemberRequest,
            create_chat_member, delete_chat_member, get_chat_members, update_chat_member,
        },
        friends::get_friends,
    },
    models::chat::{ChatMember, ChatMemberRole},
    utils::{API_BASE_URL, get_current_user_id},
};
use leptos::prelude::*;
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "chat_members_panel.module.scss");

#[component]
pub fn ChatMembersPanel(chat_id: Uuid, show: ReadSignal<bool>) -> impl IntoView {
    let show_invite_modal = RwSignal::new(false);
    let show_edit_member_modal = RwSignal::new(None::<ChatMember>);
    let selected_role = RwSignal::new(ChatMemberRole::Member);
    let new_member_name = RwSignal::new(String::new());

    let current_user_id = get_current_user_id().unwrap_or_default();

    let friends_res =
        LocalResource::new(
            move || async move { get_friends(None, Some(50)).await.unwrap_or_default() },
        );

    let chat_members =
        LocalResource::new(
            move || async move { get_chat_members(chat_id).await.unwrap_or_default() },
        );

    let create_member_action = Action::new_local(move |req: &CreateChatMemberRequest| {
        let req = req.clone();
        async move {
            if let Ok(new_member) = create_chat_member(req).await {
                chat_members.update(|m| {
                    if let Some(members) = m.as_mut() {
                        members.push(new_member)
                    }
                });
            }
        }
    });

    let update_member_action = Action::new_local(move |req: &UpdateChatMemberRequest| {
        let req = req.clone();
        async move {
            if let Ok(updated_member) = update_chat_member(req).await {
                chat_members.update(|m| {
                    if let Some(members) = m.as_mut() {
                        if let Some(member) = members
                            .iter_mut()
                            .find(|mem| mem.user_id == updated_member.user_id)
                        {
                            *member = updated_member;
                        }
                    }
                });
            }
        }
    });

    let delete_member_action = Action::new_local(move |req: &DeleteChatMemberRequest| {
        let req = req.clone();
        async move {
            if delete_chat_member(req.clone()).await.is_ok() {
                chat_members.update(|m| {
                    if let Some(members) = m.as_mut() {
                        members.retain(|mem| mem.user_id != req.member_id)
                    }
                });
            }
        }
    });

    let current_user_role = Memo::new(move |_| {
        chat_members
            .get()
            .unwrap_or_default()
            .iter()
            .find(|m| m.user_id == current_user_id)
            .map(|m| m.role.clone())
    });

    view! {
        <div class=move || format!("{} {}", style::members_panel, if show.get() { style::show } else { "" })>
            <div class=style::panel_header>
                <div class=style::header_actions_left>
                    <button
                        class=style::header_button
                        on:click=move |_| show_invite_modal.set(true)
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M14 14.252V22H4C4 17.5817 7.58172 14 12 14C12.6906 14 13.3608 14.0875 14 14.252ZM12 13C8.685 13 6 10.315 6 7C6 3.685 8.685 1 12 1C15.315 1 18 3.685 18 7C18 10.315 15.315 13 12 13ZM18 17V14H20V17H23V19H20V22H18V19H15V17H18Z"></path></svg>
                    </button>
                </div>
                <div class=style::title_container>
                    <h3>"Участники"</h3>
                    <span class=style::member_count>{move || chat_members.get().unwrap_or_default().len()}</span>
                </div>
                <div class=style::header_actions_right>
                    <button class=style::header_button on:click=move |_| {
                        delete_member_action.dispatch(DeleteChatMemberRequest { chat_id, member_id: current_user_id });
                    }>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M6.45455 19L2 22.5V4C2 3.44772 2.44772 3 3 3H21C21.5523 3 22 3.44772 22 4V18C22 18.5523 21.5523 19 21 19H6.45455ZM13.4142 11L15.8891 8.52513L14.4749 7.11091L12 9.58579L9.52513 7.11091L8.11091 8.52513L10.5858 11L8.11091 13.4749L9.52513 14.8891L12 12.4142L14.4749 14.8891L15.8891 13.4749L13.4142 11Z"></path></svg>
                    </button>
                </div>
            </div>
            <ul class=style::members_list>
                <Suspense>
                    <For
                        each=move || chat_members.get().unwrap_or_default()
                        key=|member| member.id
                        children=move |member| {
                            let member_clone = RwSignal::new(member.clone());
                            let current_role = current_user_role.get_untracked().unwrap_or_default();

                            let can_edit_role = Memo::new(move |_|
                                current_role == ChatMemberRole::Owner && member.user_id != current_user_id
                            );

                            let can_edit_name = Memo::new(move |_|
                                member.user_id == current_user_id ||
                                current_role == ChatMemberRole::Owner ||
                                (current_role == ChatMemberRole::Administrator && member.role != ChatMemberRole::Owner)
                            );

                            let can_delete = Memo::new(move |_|
                                (current_role == ChatMemberRole::Owner && member.user_id != current_user_id) ||
                                (current_role == ChatMemberRole::Administrator && member.role != ChatMemberRole::Owner && member.role != ChatMemberRole::Administrator)
                            );

                            view! {
                                <li class=style::member_item>
                                    <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, member.user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                    <div class=style::member_info>
                                        <p class=style::member_name>{member.member_name.clone().unwrap_or_else(|| format!("{} {}", member.first_name, member.last_name))}</p>
                                        <p class=style::member_username>{format!("@{}", member.user_name)}</p>
                                        <p class=style::member_role>{format!("{:?}", member.role)}</p>
                                    </div>
                                    <div class=style::member_actions>
                                        <Show when=move || can_edit_role.get() || can_edit_name.get()>
                                            <button class=style::action_button on:click=move |_| {
                                                new_member_name.set(member_clone.get_untracked().member_name.unwrap_or_default());
                                                selected_role.set(member_clone.get_untracked().role);
                                                show_edit_member_modal.set(Some(member_clone.get_untracked()));
                                            }>
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M7.24264 17.9967H3V13.754L14.435 2.319C14.8256 1.92848 15.4587 1.92848 15.8492 2.319L18.6777 5.14743C19.0682 5.53795 19.0682 6.17112 18.6777 6.56164L7.24264 17.9967ZM3 19.9967H21V21.9967H3V19.9967Z"></path></svg>
                                            </button>
                                        </Show>
                                        <Show when=move || can_delete.get()>
                                            <button class=style::action_button on:click=move |_| {
                                                delete_member_action.dispatch(DeleteChatMemberRequest { chat_id, member_id: member.user_id });
                                            }>
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M17 6H22V8H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V8H2V6H7V3C7 2.44772 7.44772 2 8 2H16C16.5523 2 17 2.44772 17 3V6ZM9 11V17H11V11H9ZM13 11V17H15V11H13ZM9 4V6H15V4H9Z"></path></svg>
                                            </button>
                                        </Show>
                                    </div>
                                </li>
                            }
                        }
                    />
                </Suspense>
            </ul>
            <Show when=move || show_invite_modal.get()>
                <div class=style::invite_panel>
                    <div class=style::panel_header>
                        <div class=style::header_actions_left>
                            <button class=style::header_button on:click=move |_| show_invite_modal.set(false)>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M8 7V11L2 6L8 1V5H13C17.4183 5 21 8.58172 21 13C21 17.4183 17.4183 21 13 21H4V19H13C16.3137 19 19 16.3137 19 13C19 9.68629 16.3137 7 13 7H8Z"></path></svg>
                            </button>
                        </div>
                        <div class=style::title_container>
                            <h3>"Пригласить в чат"</h3>
                        </div>
                        <div class=style::header_actions_right>
                        </div>
                    </div>
                    <ul class=style::members_list>
                        <For
                            each=move || friends_res.get().unwrap_or_default()
                            key=|friend| friend.user_id
                            children=move |friend| {
                                let user_id = friend.user_id;
                                view! {
                                    <li class=style::member_item>
                                        <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                        <div class=style::member_info>
                                            <p class=style::member_name>{format!("{} {}", friend.first_name, friend.last_name)}</p>
                                            <p class=style::member_username>{format!("@{}", friend.user_name)}</p>
                                        </div>
                                        <div class=style::member_actions>
                                            <button class=style::action_button on:click=move |_| {
                                                create_member_action.dispatch(CreateChatMemberRequest {
                                                    chat_id,
                                                    member_id: user_id,
                                                    inviter_id: current_user_id,
                                                });
                                                chat_members.refetch();
                                                show_invite_modal.set(false);
                                            }>
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M11 11V5H13V11H19V13H13V19H11V13H5V11H11Z"></path></svg>
                                            </button>
                                        </div>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </Show>
            <Show when=move || show_edit_member_modal.get().is_some()>
                {move || show_edit_member_modal.get().map(|member| {
                    let member_id = member.user_id;
                    let current_role = current_user_role.get_untracked().unwrap_or_default();

                    let can_edit_role = current_role == ChatMemberRole::Owner && member.user_id != current_user_id;
                    let can_edit_name = member.user_id == current_user_id ||
                                        current_role == ChatMemberRole::Owner ||
                                        (current_role == ChatMemberRole::Administrator && member.role != ChatMemberRole::Owner);

                    view! {
                        <div class=style::edit_panel>
                            <div class=style::panel_header>
                                <div class=style::header_actions_left>
                                    <button class=style::header_button on:click=move |_| show_edit_member_modal.set(None)>
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M8 7V11L2 6L8 1V5H13C17.4183 5 21 8.58172 21 13C21 17.4183 17.4183 21 13 21H4V19H13C16.3137 19 19 16.3137 19 13C19 9.68629 16.3137 7 13 7H8Z"></path></svg>
                                    </button>
                                </div>
                                <div class=style::title_container>
                                    <h3>{format!("Редактировать {}", member.user_name)}</h3>
                                </div>
                                <div class=style::header_actions_right>
                                    <button class=style::header_button on:click=move |_| {
                                        update_member_action.dispatch(UpdateChatMemberRequest {
                                            chat_id,
                                            member_id,
                                            new_member_name: if can_edit_name { Some(new_member_name.get()) } else { None },
                                            new_role: if can_edit_role { Some(selected_role.get()) } else { None },
                                        });
                                        chat_members.refetch();
                                        show_edit_member_modal.set(None);
                                    }>
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M18 21V13H6V21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3H17L21 7V20C21 20.5523 20.5523 21 20 21H18ZM16 21H8V15H16V21Z"></path></svg>
                                    </button>
                                </div>
                            </div>

                            <div class=style::form_container>
                                <Show when=move || can_edit_name>
                                    <label>"Имя в чате"</label>
                                    <input
                                        type="text"
                                        class=style::text_input
                                        prop:placeholder="Новое имя в чате"
                                        bind:value=new_member_name
                                    />
                                </Show>

                                <Show when=move || can_edit_role>
                                    <label>"Роль"</label>
                                    <select class=style::role_select on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        selected_role.set(match value.as_str() {
                                            "Administrator" => ChatMemberRole::Administrator,
                                            _ => ChatMemberRole::Member,
                                        });
                                    }>
                                        <option value="Member" selected=member.role == ChatMemberRole::Member>"Участник"</option>
                                        <option value="Administrator" selected=member.role == ChatMemberRole::Administrator>"Администратор"</option>
                                    </select>
                                </Show>
                            </div>
                        </div>
                    }
                })}
            </Show>
        </div>
    }
}
