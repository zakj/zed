use gpui::{AnyElement, AnyView, ClickEvent, CursorStyle, ElementId, IntoElement, WindowContext};
use crate::{
    prelude::*, ButtonCommon, ButtonLike, ButtonLikeRounding, ButtonSize, ButtonStyle, ElevationIndex, IconButton, IconName
};
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct SplitButton {
    main_button: ButtonLike,
    dropdown_button: ButtonLike,
    children: SmallVec<[AnyElement; 2]>,
}

impl SplitButton {
    /// Creates a new split button with a [IconButton] as the main button.
    pub fn new_icon(id: impl Into<ElementId>, icon: IconName) -> Self {
        let id = id.into();

        let element_id = |suffix| ElementId::Name(format!("{}-{}", id, suffix).into());

        Self {
            main_button: ButtonLike::new(element_id("main"))
                .child(div().px_0p5().child(Icon::new(icon)))
                .rounding(ButtonLikeRounding::Left),
            dropdown_button: ButtonLike::new(element_id("dropdown"))
                .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall))
                .width(rems(1.).into())
                .rounding(ButtonLikeRounding::Right),
            children: SmallVec::new(),
        }
    }

    /// Sets the on_click handler for the dropdown button.
    pub fn on_dropdown_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.dropdown_button = self.dropdown_button.on_click(handler);
        self
    }

    /// Set the tooltip for the dropdown button.
    pub fn dropdown_tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.dropdown_button = self.dropdown_button.tooltip(tooltip);
        self
    }
}

impl ButtonCommon for SplitButton {
    fn id(&self) -> &ElementId {
        self.main_button.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.main_button = self.main_button.style(style);
        self.dropdown_button = self.dropdown_button.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.main_button = self.main_button.size(size);
        self.dropdown_button = self.dropdown_button.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.main_button = self.main_button.tooltip(tooltip);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.main_button = self.main_button.layer(elevation);
        self.dropdown_button = self.dropdown_button.layer(elevation);
        self
    }
}

impl Clickable for SplitButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.main_button = self.main_button.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.main_button = self.main_button.cursor_style(cursor_style);
        self
    }
}

impl Disableable for SplitButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.main_button = self.main_button.disabled(disabled);
        self.dropdown_button = self.dropdown_button.disabled(disabled);
        self
    }
}

impl ParentElement for SplitButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .child(self.main_button)
            .child(self.dropdown_button)
            // todo: add dropdown menu
    }
}
