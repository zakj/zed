use crate::{prelude::*, IconButtonShape};
use gpui::{prelude::*, AnyElement, ClickEvent};
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct NotificationToast {
    title: Option<SharedString>,
    primary_action: Option<AnyElement>,
    secondary_action: Option<AnyElement>,
    on_dismiss: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    children: SmallVec<[AnyElement; 2]>,
}

impl NotificationToast {
    /// Create a new notification toast with
    /// a primary and secondary action.
    pub fn new(on_dismiss: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        Self {
            title: None,
            primary_action: None,
            secondary_action: None,
            on_dismiss: Box::new(on_dismiss),
            children: SmallVec::new(),
        }
    }

    /// Sets a title that appears above the nptification
    /// text and any other children.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the primary action of the notification.
    pub fn primary_action<E: IntoElement>(mut self, primary_action: impl Into<Option<E>>) -> Self {
        self.primary_action = primary_action.into().map(IntoElement::into_any_element);
        self
    }

    /// Sets the secondary action of the notification.
    pub fn secondary_action<E: IntoElement>(
        mut self,
        secondary_action: impl Into<Option<E>>,
    ) -> Self {
        self.secondary_action = secondary_action.into().map(IntoElement::into_any_element);
        self
    }
}

impl ParentElement for NotificationToast {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for NotificationToast {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(cx);

        v_flex()
            .text_ui(cx)
            .relative()
            .font(ui_font)
            .w_full()
            .max_w(px(360.))
            .overflow_hidden()
            .elevation_2(cx)
            .p_2p5()
            .gap_1()
            .child(
                h_flex().absolute().top_0().right_0().p_1p5().child(
                    IconButton::new("dismiss_toast", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .on_click(self.on_dismiss),
                ),
            )
            .when_some(self.title, |this, title| {
                this.child(
                    Label::new(title)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .line_height_style(LineHeightStyle::UiLabel),
                )
            })
            .children(self.children)
            .child(
                h_flex()
                    .justify_between()
                    // Left side
                    .child(
                        // Empty for now but optional elements like checkboxes
                        // might be added here
                        //
                        // Example:
                        //
                        // A extension reccomendation might have a checkbox
                        // to "Suggest extensions" that can be toggled
                        div(),
                    )
                    // Right side
                    .child(
                        h_flex()
                            .gap_1p5()
                            .when_some(self.secondary_action, |this, secondary_action| {
                                this.child(secondary_action)
                            })
                            .when_some(self.primary_action, |this, primary_action| {
                                this.child(primary_action)
                            }),
                    ),
            )
    }
}
