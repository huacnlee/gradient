mod gradient;

use std::sync::Arc;

use gpui::{
    px, relative, Edges, Element, Hsla, Interactivity, IntoElement, Pixels, RenderImage, Size,
    Style, WindowContext,
};
pub use gradient::*;

/// Render A Gradient
pub struct GradientElement {
    interactivity: Interactivity,
    base: Gradient,
    angle_or_corner: AngleOrCorner,
    cached_size: Option<Size<Pixels>>,
    cache: Option<Arc<RenderImage>>,
}

impl GradientElement {
    pub fn linear() -> Self {
        Self {
            interactivity: Interactivity::default(),
            base: Gradient::default(),
            angle_or_corner: AngleOrCorner::Angle(0.0),
            cache: None,
            cached_size: None,
        }
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.angle_or_corner = AngleOrCorner::Angle(angle);
        self
    }

    pub fn side(mut self, side: GradientSide) -> Self {
        self.angle_or_corner = AngleOrCorner::Side(side);
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.base.colors.push(color_stop(color.into(), None));
        self
    }

    pub fn color_with_percentage(mut self, color: impl Into<Hsla>, percentage: f32) -> Self {
        self.base
            .colors
            .push(color_stop(color.into(), Some(percentage)));
        self
    }

    pub fn render_image(&mut self, size: Size<Pixels>) -> Arc<RenderImage> {
        let (start, end) = Gradient::calculate_start_end(self.angle_or_corner, size);
        if let Some(cache) = &self.cache {
            if self.cached_size == Some(size) {
                return cache.clone();
            }
        }

        self.base.start = start;
        self.base.end = end;
        let image = self.base.render(size);
        let image = Arc::new(image);
        self.cached_size = Some(size);
        self.cache = Some(image.clone());
        image
    }
}

impl IntoElement for GradientElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for GradientElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        cx: &mut gpui::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.0;
        style.flex_shrink = 1.0;
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        let id = cx.request_layout(style, []);
        (id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: gpui::Bounds<gpui::Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut WindowContext,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        cx: &mut gpui::WindowContext,
    ) {
        cx.paint_quad(gpui::PaintQuad {
            bounds,
            corner_radii: px(0.).into(),
            background: gpui::transparent_white(),
            border_widths: Edges::all(px(1.)),
            border_color: gpui::blue(),
        });

        let image = self.render_image(bounds.size);
        match cx.paint_image(bounds, px(0.).into(), image, 0, false) {
            Ok(_) => {}
            Err(err) => eprintln!("failed to paint GradientElement: {:?}", err),
        }
    }
}
