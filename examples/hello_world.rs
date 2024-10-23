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
                    .side(GradientSide::Right)
                    .color_with_percentage(hsla(30.0 / 360.0, 1.0, 0.5, 1.0), 0.2)
                    .color_with_percentage(hsla(90.0 / 360.0, 1.0, 0.5, 1.0), 0.90),
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
