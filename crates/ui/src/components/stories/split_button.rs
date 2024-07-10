use gpui::Render;
use story::Story;

use crate::{prelude::*, IconName, SplitButton};

pub struct SplitButtonStory;

impl Render for SplitButtonStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {

        Story::container()
            .child(Story::title_for::<SplitButton>())
            .child(Story::label("Default"))
            .child(SplitButton::new_icon("default_filled",IconName::Hash))
    }
}
