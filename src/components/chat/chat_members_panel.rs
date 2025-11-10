use crate::{
    api::{
        chat_members::{
            CreateChatMemberRequest, DeleteChatMemberRequest, UpdateChatMemberRequest,
            create_chat_member, delete_chat_member, get_chat_members, update_chat_member,
        },
        friends::get_friends,
    },
    models::{
        chat::{ChatMember, ChatMemberRole},
        friends::Friendship,
    },
    utils::{API_BASE_URL, get_current_user_id},
};
use leptos::prelude::*;
use leptos_use::{UseTimeoutFnReturn, use_timeout_fn};
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "chat_members_panel.module.scss");

#[component]
pub fn ChatMembersPanel(chat_id: Uuid) -> impl IntoView {
    let user_to_invite = RwSignal::new(None::<Uuid>);
    let show_invite_modal = RwSignal::new(false);
    let show_edit_role_modal = RwSignal::new(None::<ChatMember>);
    let selected_role = RwSignal::new(ChatMemberRole::Member);
    let show_anim = RwSignal::new(false);

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        move |_| {
            show_anim.set(true);
        },
        10.0,
    );
    start(());

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

    let can_manage = Memo::new(move |_| {
        matches!(
            current_user_role.get(),
            Some(ChatMemberRole::Owner) | Some(ChatMemberRole::Administrator)
        )
    });

    view! {
        <div class=move || format!("{} {}", style::members_panel, if show_anim.get() { style::show } else { "" })>
            <div class=style::panel_header>
                <div class=style::header_actions_left>
                    <button class=style::header_button on:click=move |_| {
                        delete_member_action.dispatch(DeleteChatMemberRequest { chat_id, member_id: current_user_id });
                    }>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM12 20V15M8.00002 15L12 11L16 15M12 4V9"></path></svg>
                    </button>
                </div>
                <h3>"Участники"</h3>
                <div class=style::header_actions_right>
                    <button
                        class=style::header_button
                        on:click=move |_| show_invite_modal.set(true)
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M14 2V4H12V2H14ZM12 20H14V22H12V20ZM20 12V14H22V12H20ZM2 12V14H4V12H2ZM18.1924 16.7782L19.6066 18.1924L16.7782 21.0208L15.364 19.6066L18.1924 16.7782ZM12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4 12.5523 4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13 19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11H13V5C13 4.44772 12.5523 4 12 4ZM19.6066 4.3934L21.0208 5.80761L18.1924 8.63604L16.7782 7.22183L19.6066 4.3934ZM8.63604 18.1924L7.22183 19.6066L4.3934 16.7782L5.80761 15.364L8.63604 18.1924ZM7.22183 5.80761L5.80761 4.3934L8.63604 1.565L10.0503 2.97921L7.22183 5.80761Z"></path></svg>
                    </button>
                </div>
            </div>
            <ul class=style::members_list>
                <Suspense>
                    <For
                        each=move || chat_members.get().unwrap_or_default()
                        key=|member| member.id
                        children=move |member| {
                            let member_user_id = member.user_id;
                            let member = RwSignal::new(member);

                            view! {
                                <li class=style::member_item>
                                    <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, member.get_untracked().user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                    <div class=style::member_info>
                                        <p class=style::member_name>{format!("{} {}", member.get_untracked().first_name, member.get_untracked().last_name)}</p>
                                        <p class=style::member_username>{format!("@{}", member.get_untracked().member_name.unwrap_or(member.get_untracked().user_name))}</p>
                                        <p class=style::member_role>{format!("{:?}", member.get_untracked().role)}</p>
                                    </div>
                                    <Show when=move || can_manage.get() && member_user_id != current_user_id>
                                         <div class=style::member_actions>
                                            <button class=style::action_button on:click=move |_| {
                                                selected_role.set(member.get_untracked().role);
                                                show_edit_role_modal.set(Some(member.get_untracked()));
                                            }>
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12.9,2.6l8.5,8.5l-2.8,2.8l-8.5-8.5L12.9,2.6z M4,14.1V18h3.9l10-10L14,4.1L4,14.1z M4,20v2h16v-2H4z"></path></svg>
                                            </button>
                                            <button class=style::action_button on:click=move |_| {
                                                delete_member_action.dispatch(DeleteChatMemberRequest { chat_id, member_id: member_user_id });
                                            }>
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M7 4V2H17V4H22V6H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V6H2V4H7ZM6 6V20H18V6H6ZM9 9H11V17H9V9ZM13 9H15V17H13V9Z"></path></svg>
                                            </button>
                                         </div>
                                    </Show>
                                </li>
                            }
                        }
                    />
                </Suspense>
            </ul>

            <Show when=move || show_invite_modal.get()>
                <div class=style::invite_modal>
                    <div class=style::modal_content>
                        <h3>"Пригласить пользователя"</h3>
                        <div class=style::user_list>
                            <For
                                each=move || friends_res.get().unwrap_or_default()
                                key=|friend| friend.user_id
                                children=move |friend| {
                                    let user_id = friend.user_id;
                                    view! {
                                        <div
                                            class=move || format!("{} {}", style::user_item, if user_to_invite.get() == Some(user_id) { style::selected } else { "" })
                                            on:click=move |_| user_to_invite.set(Some(user_id))
                                        >
                                            <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                            <span>{format!("{} {}", friend.first_name, friend.last_name)}</span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                        <div class=style::modal_actions>
                            <button class=style::cancel_button on:click=move |_| show_invite_modal.set(false)>"Отмена"</button>
                            <button class=style::save_button on:click=move |_| {
                                if let Some(member_id) = user_to_invite.get() {
                                    create_member_action.dispatch(CreateChatMemberRequest {
                                        chat_id,
                                        member_id,
                                        inviter_id: current_user_id,
                                    });
                                    show_invite_modal.set(false);
                                    user_to_invite.set(None);
                                }
                            }>"Пригласить"</button>
                        </div>
                    </div>
                </div>
            </Show>

            <Show when=move || show_edit_role_modal.get().is_some()>
                {move || show_edit_role_modal.get().map(|member| {
                    let member_id = member.user_id;
                    view! {
                        <div class=style::edit_role_modal>
                            <div class=style::modal_content>
                                <h3>{format!("Изменить роль для {}", member.user_name)}</h3>
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
                                <div class=style::modal_actions>
                                    <button class=style::cancel_button on:click=move |_| show_edit_role_modal.set(None)>"Отмена"</button>
                                    <button class=style::save_button on:click=move |_| {
                                        update_member_action.dispatch(UpdateChatMemberRequest {
                                            chat_id,
                                            member_id,
                                            new_member_name: None,
                                            new_role: Some(selected_role.get()),
                                        });
                                        show_edit_role_modal.set(None);
                                    }>"Сохранить"</button>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </Show>
        </div>
    }
}
