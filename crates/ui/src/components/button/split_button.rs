use gpui::{AnyElement, AnyView, ClickEvent, CursorStyle, ElementId, IntoElement, View, WindowContext};
use crate::{
    prelude::*, ButtonCommon, ButtonLike, ButtonLikeRounding, ButtonSize, ButtonStyle, ContextMenu, ElevationIndex, IconButton, IconName, PopoverMenu
};
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct SplitButton {
    button: ButtonLike,
    dropdown_menu: AnyElement,
    children: SmallVec<[AnyElement; 2]>,
}

impl SplitButton {
    /// Creates a new split button with a [IconButton] as the main button.
    pub fn new_icon(id: impl Into<ElementId>, icon: IconName) -> Self {
        let id = id.into();

        let element_id = |suffix| ElementId::Name(format!("{}-{}", id, suffix).into());

        let dropdown_menu = PopoverMenu::new(element_id("menu"))
            .menu(move |cx| {
                ContextMenu::build(cx, move |menu, _cx| {
                    menu.header("REPL")
                }).into()
            }).trigger(ButtonLike::new(element_id("dropdown"))
                .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall))
                .width(rems(1.).into())
                .rounding(ButtonLikeRounding::Right));


        Self {
            button: ButtonLike::new(element_id("main"))
                .child(div().px_0p5().child(Icon::new(icon)))
                .rounding(ButtonLikeRounding::Left),
            dropdown_menu: dropdown_menu.into_any_element(),
            children: SmallVec::new(),
        }
    }
}

impl ButtonCommon for SplitButton {
    fn id(&self) -> &ElementId {
        self.button.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.button = self.button.style(style);
        self.popover_button = self.popover_button.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.button = self.button.size(size);
        self.popover_button = self.popover_button.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.button = self.button.tooltip(tooltip);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.button = self.button.layer(elevation);
        self.popover_button = self.popover_button.layer(elevation);
        self
    }
}

impl Clickable for SplitButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.button = self.button.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.button = self.button.cursor_style(cursor_style);
        self
    }
}

impl Disableable for SplitButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self.popover_button = self.popover_button.disabled(disabled);
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
            .child(self.button)
            .child(self.popover_button)
            // todo: add dropdown menu
    }
}
