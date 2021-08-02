use gpui::{
    color::ColorU,
    fonts::{Properties, Weight},
    DebugContext, Element as _, Quad,
};
use log::LevelFilter;
use pathfinder_geometry::rect::RectF;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(|_| TextView);
    });
}

struct TextView;
struct TextElement;

impl gpui::Entity for TextView {
    type Event = ();
}

impl gpui::View for TextView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&self, _: &gpui::RenderContext<Self>) -> gpui::ElementBox {
        TextElement.boxed()
    }
}

impl gpui::Element for TextElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut gpui::LayoutContext,
    ) -> (pathfinder_geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn after_layout(
        &mut self,
        _: pathfinder_geometry::vector::Vector2F,
        _: &mut Self::LayoutState,
        _: &mut gpui::AfterLayoutContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let font_size = 12.;
        let family = cx.font_cache.load_family(&["SF Pro Display"]).unwrap();
        let normal = cx
            .font_cache
            .select_font(family, &Default::default())
            .unwrap();
        let bold = cx
            .font_cache
            .select_font(
                family,
                &Properties {
                    weight: Weight::BOLD,
                    ..Default::default()
                },
            )
            .unwrap();

        let text = "Hello world!";
        let line = cx.text_layout_cache.layout_str(
            text,
            font_size,
            &[
                (1, normal, ColorU::default()),
                (1, bold, ColorU::default()),
                (1, normal, ColorU::default()),
                (1, bold, ColorU::default()),
                (text.len() - 4, normal, ColorU::default()),
            ],
        );

        cx.scene.push_quad(Quad {
            bounds: bounds,
            background: Some(ColorU::white()),
            ..Default::default()
        });
        line.paint(bounds.origin(), bounds, cx);
    }

    fn dispatch_event(
        &mut self,
        _: &gpui::Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut gpui::EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> gpui::json::Value {
        todo!()
    }
}
