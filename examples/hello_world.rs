use gpui::*;
use gpui_gradient::{GradientElement, GradientSide};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(gpui::white())
            .size_full()
            .justify_center()
            .items_center()
            .child(
                GradientElement::linear()
                    .side(GradientSide::BottomRight)
                    .color_with_percentage(hsla(30.0 / 360.0, 1.0, 0.5, 1.0), 0.0)
                    .color_with_percentage(hsla(60.0 / 360.0, 1.0, 0.5, 1.0), 0.14)
                    .color_with_percentage(hsla(120.0 / 360.0, 1.0, 0.5, 1.0), 0.29)
                    .color_with_percentage(hsla(240.0 / 360.0, 1.0, 0.5, 1.0), 0.43)
                    .color_with_percentage(hsla(275.0 / 360.0, 1.0, 0.5, 1.0), 0.71)
                    .color_with_percentage(hsla(300.0 / 360.0, 1.0, 0.5, 1.0), 0.86)
                    .color_with_percentage(hsla(330.0 / 360.0, 1.0, 0.5, 1.0), 1.0),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| HelloWorld {}),
        )
        .unwrap();
    });
}
