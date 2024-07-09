use gpui::{Render, ViewContext};
use story::Story;

use crate::{prelude::*, NotificationToast};

pub struct NotificationToastStory;

impl Render for NotificationToastStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<NotificationToast>())
            .child(Story::label("With headline"))
            .child(
                div().p_4().child(
                    NotificationToast::new(|_, _cx| {})
                        .title("Recommended Extension")
                        .child(Label::new("Do you want to install the recommended 'astro' extension for 'astro' files?"))
                        .primary_action(Button::new("install", "Install"))
                        .secondary_action(Button::new("dismiss", "Don't Install").color(Color::Muted))
                )
            )
    }
}
