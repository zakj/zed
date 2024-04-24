#![allow(unused, dead_code)]
//! # UI – Chat List

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use chrono::NaiveDateTime;
use client::{User, UserStore};
use editor::*;
use gpui::*;
use indoc::indoc;
use language::{language_settings::SoftWrap, Buffer, LanguageRegistry, ToOffset as _};
use nanoid::nanoid;
use rich_text::RichText;
use settings::Settings;
use static_chat::static_chat;
use theme::ThemeSettings;
use ui::{
    utils::{format_distance_from_now, DateTimeType},
    *,
};
use workspace::Workspace;

mod static_chat;

// TODO next
//
// ## Chat Messsage
// - [ ] render ChatMessages as RichText
//  - [x] get the LanguageRegistry on to ChatList
//  - [x] either pass a RichText to ChatMessage, or create the RichText from string in ChatMessage
//  - [ ] split ChatMessage into new_user and new_assistant
// - [ ] Build rendering for ChatContext
// - [ ] Hook up message collapsing
//
// ## Chat List
// - [x] render a list of chat messages
//
// ## Composer
// - build out composer static UI
// - add editor
// - add button on_click actions for Send and Quote Selection
// - add model switcher

pub struct ChatListStore {
    collapsed_messages: HashMap<Arc<str>, bool>,
}

pub struct ChatList {
    workspace: WeakView<Workspace>,
    languages: Arc<LanguageRegistry>,
    user_store: Model<UserStore>,
    chat_store: Model<ChatListStore>,
    messages: Vec<(Arc<str>, ChatRole, Arc<str>, chrono::NaiveDateTime)>,
    composer: View<Composer>,
}

impl ChatList {
    pub fn new(
        workspace: WeakView<Workspace>,
        user_store: Model<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Result<ChatList> {
        let user = user_store.read(cx).current_user();

        let composer = cx.new_view(|_| Composer {
            player: user.clone(),
        });

        let workspace_handle = workspace.clone();

        workspace.update(cx, |workspace, cx| Self {
            user_store,
            languages: workspace.app_state().languages.clone(),
            workspace: workspace_handle,
            messages: static_chat(),
            chat_store: cx.new_model(|cx| ChatListStore {
                collapsed_messages: HashMap::default(),
            }),
            composer,
        })
    }

    pub fn current_user(&self, cx: &ViewContext<Self>) -> Option<Arc<User>> {
        self.user_store.read(&cx).current_user()
    }

    pub fn assistant_user() -> User {
        User {
            id: 99999,
            github_login: "Assistant".into(),
            avatar_uri: "https://zed.dev/assistant_avatar.png".into(),
        }
    }
}

impl Render for ChatList {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(current_user) = self.current_user(cx) else {
            return div().id("empty").child("Loading...");
        };

        let chat_store_handle = self.chat_store.clone();

        let messages = self.messages.iter().map(|(id, role, message, datetime)| {
            let user = match role {
                ChatRole::User => current_user.clone(),
                ChatRole::Assistant => Arc::new(Self::assistant_user()),
            };

            let rich_text =
                rich_text::render_rich_text(message.to_string(), &[], &self.languages, None);

            ChatMessage::new(id.clone(), *role, user, datetime.clone(), rich_text, {
                let id = id.clone();
                let chat_store_handle = chat_store_handle.clone();
                Box::new(move |collapsed, cx| {
                    println!("Collapsing {id}: {collapsed}");
                    cx.update_model(&chat_store_handle, |chat_store, cx| {
                        println!("Updating model for {id}");
                        let mut entry =
                            chat_store.collapsed_messages.entry(id.clone()).or_default();
                        *entry = collapsed;
                    });
                })
            })
            .collapsed(
                self.chat_store
                    .read(cx)
                    .collapsed_messages
                    .contains_key(id.as_ref()),
            )
        });

        div()
            .id("chat-list")
            .size_full()
            .overflow_y_scroll()
            .on_click(|_event, _cx| println!("Clicked chat list"))
            .bg(cx.theme().colors().background)
            .child(
                v_flex()
                    .mx_auto()
                    .justify_between()
                    .min_h_full()
                    .max_w(rems(42.0))
                    .gap_2()
                    .p_4()
                    .child(v_flex().gap_2().children(messages))
                    .child(self.composer.clone()),
            )
    }
}

pub struct Composer {
    player: Option<Arc<User>>,
}

impl Render for Composer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut player_avatar = div().into_any_element();
        if let Some(player) = self.player.clone() {
            player_avatar = Avatar::new(player.avatar_uri.clone())
                .size(rems(20.0 / 16.0))
                .into_any_element();
        }

        h_flex()
            .w_full()
            .items_start()
            .gap_3()
            .child(player_avatar)
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .overflow_hidden()
                            .w_full()
                            .p_4()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_lg()
                            .child(
                                v_flex()
                                    .w_full()
                                    .gap_1()
                                    .child(
                                        // Editor
                                        div().w_full().min_h(rems(80.0 / 16.0)).child(
                                            div()
                                                .text_color(cx.theme().colors().text_placeholder)
                                                .child("placeholder"),
                                        ),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .justify_between()
                                            .w_full()
                                            .child(
                                                h_flex()
                                                    .gap_1()
                                                    .child(
                                                        IconButton::new(
                                                            "add-context",
                                                            IconName::FileDoc,
                                                        )
                                                        .icon_color(Color::Muted),
                                                    )
                                                    .child(
                                                        IconButton::new(
                                                            "add-context",
                                                            IconName::Plus,
                                                        )
                                                        .icon_color(Color::Muted),
                                                    ),
                                            )
                                            .child(
                                                // Send button
                                                Button::new("send-button", "Send")
                                                    .style(ButtonStyle::Filled),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(Button::new("swich-model", "gpt-4-turbo").color(Color::Muted))
                            .child(Button::new("quote", "Quote Selection").color(Color::Muted)),
                    ),
            )
    }
}

// === Chat Header ===

#[derive(Debug, Clone, Copy)]
pub enum ChatRole {
    User,
    Assistant,
}

pub enum ChatContextType {
    Code,
    Diagnostic,
    Docs,
}

pub struct ChatContext {
    context_type: ChatContextType,
    content: String,
}

struct ChatContexts {
    contexts: Vec<ChatContext>,
}

#[derive(IntoElement)]
pub struct ChatHeader {
    role: ChatRole,
    player: Arc<User>,
    sent_at: String,
    contexts: Vec<String>,
}

impl ChatHeader {
    pub fn new(role: ChatRole, player: Arc<User>, sent_at: String) -> ChatHeader {
        ChatHeader {
            role,
            player,
            sent_at,
            contexts: Vec::new(),
        }
    }

    pub fn role(&mut self, role: ChatRole) -> &mut Self {
        self.role = role;
        self
    }

    pub fn player(&mut self, player: Arc<User>) -> &mut Self {
        self.player = player;
        self
    }

    pub fn sent_at(&mut self, sent_at: String) -> &mut Self {
        self.sent_at = sent_at;
        self
    }

    pub fn contexts(&mut self, contexts: Vec<String>) -> &mut Self {
        self.contexts = contexts;
        self
    }
}

impl RenderOnce for ChatHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let player_avatar = Avatar::new(self.player.avatar_uri.clone()).size(rems(20.0 / 16.0));
        let player_name = Label::new(self.player.github_login.clone()).color(Color::Default);
        let sent_at = Label::new(self.sent_at).color(Color::Muted);

        h_flex()
            .justify_between()
            .child(
                h_flex()
                    .gap_3()
                    .child(player_avatar)
                    .child(player_name)
                    .child(sent_at),
            )
            .child(div().when(self.contexts.len() > 0, |this| {
                this.child(Label::new(self.contexts.len().to_string()).color(Color::Muted))
                // this.child(Button::new("View Contexts")))
            }))
    }
}

#[derive(IntoElement)]
pub struct ChatMessage {
    id: Arc<str>,
    role: ChatRole,
    player: Arc<User>,
    sent_at: NaiveDateTime,
    message: RichText,
    collapsed: bool,
    on_collapse: Box<dyn Fn(bool, &mut WindowContext) + 'static>,
}

impl ChatMessage {
    pub fn new(
        id: Arc<str>,
        role: ChatRole,
        player: Arc<User>,
        sent_at: NaiveDateTime,
        message: RichText,
        on_collapse: Box<dyn Fn(bool, &mut WindowContext) + 'static>,
    ) -> ChatMessage {
        ChatMessage {
            id,
            role,
            player,
            sent_at,
            message,
            collapsed: false,
            on_collapse,
        }
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for ChatMessage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        // TODO: This should be top padding + 1.5x line height
        // Set the message height to cut off at exactly 1.5 lines when collapsed
        let collapsed_height = rems(2.875);
        let sent_at = format_distance_from_now(DateTimeType::Naive(self.sent_at), true, true, true);

        let header = ChatHeader::new(self.role, self.player, sent_at);
        let collapse_handle_id = SharedString::from(format!("{}_collapse_handle", self.id.clone()));
        let collapse_handle = h_flex()
            .id(collapse_handle_id.clone())
            .group(collapse_handle_id.clone())
            .flex_none()
            .justify_center()
            // .debug_bg_red()
            .w_1()
            .mx_2()
            .h_full()
            .on_click(move |_event, cx| (self.on_collapse)(!self.collapsed, cx))
            .child(
                div()
                    .w_px()
                    .h_full()
                    .rounded_lg()
                    .overflow_hidden()
                    .bg(cx.theme().colors().element_background)
                    .group_hover(collapse_handle_id, |this| {
                        this.bg(cx.theme().colors().element_hover)
                    }),
            );
        let content = div()
            .overflow_hidden()
            .w_full()
            .p_4()
            .rounded_lg()
            .when(self.collapsed, |this| this.h(collapsed_height))
            .bg(cx.theme().colors().surface_background)
            .child(self.message.element("message".into(), cx));

        v_flex()
            .gap_1()
            .child(header)
            .child(h_flex().gap_3().child(collapse_handle).child(content))
    }
}
