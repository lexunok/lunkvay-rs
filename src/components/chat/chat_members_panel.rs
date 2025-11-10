use crate::{
    api::{
        chat_members::{
            create_chat_member, delete_chat_member, get_chat_members, update_chat_member,
            CreateChatMemberRequest, DeleteChatMemberRequest, UpdateChatMemberRequest,
        },
        friends::get_friends,
    },
    models::{
        chat::{ChatMember, ChatMemberRole},
    },
    utils::{API_BASE_URL,get_current_user_id},
};
use leptos::prelude::*;
use stylance::import_style;
use uuid::Uuid;

import_style!(style, "chat_members_panel.module.scss");

#[component]
pub fn ChatMembersPanel(
    chat_id: Uuid
) -> impl IntoView {

    let user_to_invite= RwSignal::new(None::<Uuid>);
    let selected_role = RwSignal::new(ChatMemberRole::Member);

    let current_user_id = get_current_user_id().unwrap_or_default();

    let friends_res = LocalResource::new(move || get_friends(None, Some(50)));
    
    let chat_members = LocalResource::new(move || async move {
        get_chat_members(chat_id).await.unwrap_or_default()
    });

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
                        if let Some(member) = members.iter_mut().find(|mem| mem.user_id == updated_member.user_id) {
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
        chat_members.get().unwrap_or_default().iter().find(|m| m.user_id == current_user_id).map(|m| m.role.clone())
    });

    let can_manage = Memo::new(move |_| {
        matches!(current_user_role.get(), Some(ChatMemberRole::Owner) | Some(ChatMemberRole::Administrator))
    });

    view! {
        <div class=move || format!("{} {}", style::members_panel, style::show)>
            <div class=style::panel_header>
                <h3>"Участники"</h3>
            </div>
            <ul class=style::members_list>
                <Suspense>
                    <For
                        each=move || chat_members.get().unwrap_or_default()
                        key=|member| member.id
                        children=move |member| {
                            let member_user_id = member.user_id;
                            view! {
                                <li class=style::member_item>
                                    <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, member.user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
                                    <div class=style::member_info>
                                        <p class=style::member_name>{format!("{} {}", member.first_name, member.last_name)}</p>
                                        <p class=style::member_role>{format!("{:?}", member.role)}</p>
                                    </div>
                                    <Show when=move || can_manage.get() && member_user_id != current_user_id>
                                        <div class=style::member_actions>
                                            <button class=style::action_button >
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
            <div class=style::panel_footer>
                <Show when=move || can_manage.get()>
                    <button class=format!("{} {}", style::action_button, style::invite_button)>
                        "Пригласить"
                    </button>
                </Show>
                <button class=format!("{} {}", style::action_button, style::leave_button) on:click=move |_| {
                    delete_member_action.dispatch(DeleteChatMemberRequest { chat_id, member_id: current_user_id });
                }>
                    "Выйти из группы"
                </button>
            </div>

            // <Show when=move || show_invite_modal.get()>
            //     <div class=style::invite_modal>
            //         <div class=style::modal_content>
            //             <h3>"Пригласить пользователя"</h3>
            //             <div class=style::user_list>
            //                 <For
            //                     each=move || possible_friends.get()
            //                     key=|user| user.user_id
            //                     children=move |user| {
            //                         let user_id = user.user_id;
            //                         view! {
            //                             <div
            //                                 class=move || format!("{} {}", style::user_item, if selected_user_to_invite.get() == Some(user_id) { style::selected } else { "" })
            //                                 on:click=move |_| set_selected_user_to_invite.set(Some(user_id))
            //                             >
            //                                 <img class=style::avatar src=format!("{}/avatar/{}", API_BASE_URL, user.user_id) onerror="this.onerror=null;this.src='/images/userdefault.webp';"/>
            //                                 <span>{format!("{} {}", user.first_name, user.last_name.unwrap_or_default())}</span>
            //                             </div>
            //                         }
            //                     }
            //                 />
            //             </div>
            //             <div class=style::modal_actions>
            //                 <button class=style::cancel_button on:click=move |_| set_show_invite_modal.set(false)>"Отмена"</button>
            //                 <button class=style::save_button on:click=move |_| {
            //                     if let Some(member_id) = selected_user_to_invite.get() {
            //                         create_member_action.dispatch(CreateChatMemberRequest {
            //                             chat_id,
            //                             member_id,
            //                             inviter_id: current_user_id,
            //                         });
            //                     }
            //                 }>"Пригласить"</button>
            //             </div>
            //         </div>
            //     </div>
            // </Show>

            // <Show when=move || show_edit_role_modal.get().is_some()>
            //     {move || show_edit_role_modal.get().map(|member| {
            //         let member_id = Uuid::parse_str(&member.user_id).unwrap_or_default();
            //         view! {
            //             <div class=style::edit_role_modal>
            //                 <div class=style::modal_content>
            //                     <h3>{format!("Изменить роль для {}", member.user_name)}</h3>
            //                     <select class=style::role_select on:change=move |ev| {
            //                         let value = event_target_value(&ev);
            //                         set_selected_role.set(match value.as_str() {
            //                             "Administrator" => ChatMemberRole::Administrator,
            //                             _ => ChatMemberRole::Member,
            //                         });
            //                     }>
            //                         <option value="Member" selected=member.role == ChatMemberRole::Member>"Member"</option>
            //                         <option value="Administrator" selected=member.role == ChatMemberRole::Administrator>"Administrator"</option>
            //                     </select>
            //                     <div class=style::modal_actions>
            //                         <button class=style::cancel_button on:click=move |_| set_show_edit_role_modal.set(None)>"Отмена"</button>
            //                         <button class=style::save_button on:click=move |_| {
            //                             update_member_action.dispatch(UpdateChatMemberRequest {
            //                                 chat_id,
            //                                 member_id,
            //                                 new_member_name: None,
            //                                 new_role: Some(selected_role.get()),
            //                             });
            //                         }>"Сохранить"</button>
            //                     </div>
            //                 </div>
            //             </div>
            //         }
            //     })}
            // </Show>
        </div>
    }
}
