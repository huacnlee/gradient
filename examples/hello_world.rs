use gpui::*;
use gpui_gradient::GradientElement;

struct HelloWorld {
    seed: f64,
}

impl HelloWorld {
    fn new() -> Self {
        let seed = rand::random::<f64>() * 360.0;

        Self { seed }
    }

    fn create_gradient_element(&self, ix: usize) -> GradientElement {
        // Rand seed from 0.0 .. 360.0

        let angle = ((ix as f64 * self.seed) % 360.0) as f32;
        GradientElement::linear()
            .angle(angle)
            .color_with_percentage(hsla(angle / 360.0, 1.0, 0.4, 1.0), 0.2)
            .color_with_percentage(hsla((angle + 123.0) / 360.0, 1.0, 0.6, 1.0), 0.9)
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut container = div()
            .flex()
            .flex_col()
            .bg(gpui::white())
            .size_full()
            .justify_center()
            .items_center()
            .gap_2();

        for j in 0..20 {
            let mut row = div().size_full().flex().flex_row().gap_2();
            for i in 0..20 {
                row = row.child(self.create_gradient_element(j * i));
            }
            container = container.child(row);
        }
        container
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| HelloWorld::new()),
        )
        .unwrap();
    });
}
