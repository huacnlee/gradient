use gpui::{hsla, px, Hsla, Pixels, Point, RenderImage, Size};
use image::{Frame, ImageBuffer};
use smallvec::SmallVec;

trait HslaExt {
    fn interpolate(&self, other: Hsla, t: f32) -> Hsla;
}

impl HslaExt for Hsla {
    fn interpolate(&self, other: Hsla, t: f32) -> Hsla {
        let h = self.h * (1.0 - t) + other.h * t;
        let s = self.s * (1.0 - t) + other.s * t;
        let l = self.l * (1.0 - t) + other.l * t;
        let a = self.a * (1.0 - t) + other.a * t;
        hsla(h, s, l, a)
    }
}

pub struct ColorStop {
    color: Hsla,
    percentage: Option<f32>,
}

pub fn color_stop(color: Hsla, percentage: Option<f32>) -> ColorStop {
    ColorStop { color, percentage }
}

#[derive(Clone, Copy, Default)]
pub enum GradientType {
    #[default]
    Linear,
    RepeatingLinear,
}

#[derive(Clone, Copy, Default)]
pub enum GradientSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy)]
pub enum AngleOrCorner {
    Angle(f32),
    Side(GradientSide),
}

#[derive(Default)]
pub struct Gradient {
    pub colors: Vec<ColorStop>,
    pub gradient_type: GradientType,
    pub start: Point<Pixels>,
    pub end: Point<Pixels>,
}

impl Gradient {
    pub fn new(gradient_type: GradientType, colors: Vec<ColorStop>) -> Self {
        Self {
            colors,
            gradient_type,
            ..Default::default()
        }
    }

    pub fn render(&self, size: Size<Pixels>) -> RenderImage {
        let width = size.width.0;
        let height = size.height.0;

        let mut img = ImageBuffer::new(width as u32, height as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = self.calculate_color(Point {
                x: px(x as f32),
                y: px(y as f32),
            });
            let rgba = color.to_rgb();

            // Convert from RGBA to BGRA.
            *pixel = image::Rgba([
                (rgba.b * 255.) as u8,
                (rgba.g * 255.) as u8,
                (rgba.r * 255.) as u8,
                (rgba.a * 255.) as u8,
            ]);
        }

        let data = SmallVec::from_elem(Frame::new(img), 1);
        RenderImage::new(data)
    }

    pub fn linear(
        angle_or_corner: AngleOrCorner,
        colors: Vec<ColorStop>,
        size: Size<Pixels>,
    ) -> Self {
        let (start, end) = Self::calculate_start_end(angle_or_corner, size);
        Self {
            colors,
            gradient_type: GradientType::Linear,
            start,
            end,
        }
    }

    pub fn repeating_linear(
        angle_or_corner: AngleOrCorner,
        colors: Vec<ColorStop>,
        size: Size<Pixels>,
    ) -> Self {
        let (start, end) = Self::calculate_start_end(angle_or_corner, size);
        Self {
            colors,
            gradient_type: GradientType::RepeatingLinear,
            start,
            end,
        }
    }

    pub(super) fn calculate_start_end(
        angle_or_corner: AngleOrCorner,
        size: Size<Pixels>,
    ) -> (Point<Pixels>, Point<Pixels>) {
        let width = size.width;
        let height = size.height;

        match angle_or_corner {
            AngleOrCorner::Angle(angle) => {
                let rad = angle.to_radians();
                let x = rad.cos();
                let y = rad.sin();
                let start = Point {
                    x: width * (1.0 - x) / 2.0,
                    y: height * (1.0 - y) / 2.0,
                };
                let end = Point {
                    x: width * (1.0 + x) / 2.0,
                    y: height * (1.0 + y) / 2.0,
                };
                (start, end)
            }
            AngleOrCorner::Side(side) => {
                let (start, end) = match side {
                    GradientSide::Top => (
                        Point {
                            x: width / 2.0,
                            y: height,
                        },
                        Point {
                            x: width / 2.0,
                            y: px(0.0),
                        },
                    ),
                    GradientSide::Right => (
                        Point {
                            x: px(0.0),
                            y: height / 2.0,
                        },
                        Point {
                            x: width,
                            y: height / 2.0,
                        },
                    ),
                    GradientSide::Bottom => (
                        Point {
                            x: width / 2.0,
                            y: px(0.0),
                        },
                        Point {
                            x: width / 2.0,
                            y: height,
                        },
                    ),
                    GradientSide::Left => (
                        Point {
                            x: width,
                            y: height / 2.0,
                        },
                        Point {
                            x: px(0.0),
                            y: height / 2.0,
                        },
                    ),
                    GradientSide::TopLeft => (
                        Point {
                            x: width,
                            y: height,
                        },
                        Point {
                            x: px(0.0),
                            y: px(0.0),
                        },
                    ),
                    GradientSide::TopRight => (
                        Point {
                            x: px(0.0),
                            y: height,
                        },
                        Point {
                            x: width,
                            y: px(0.0),
                        },
                    ),
                    GradientSide::BottomLeft => (
                        Point {
                            x: width,
                            y: px(0.0),
                        },
                        Point {
                            x: px(0.0),
                            y: height,
                        },
                    ),
                    GradientSide::BottomRight => (
                        Point {
                            x: px(0.0),
                            y: px(0.0),
                        },
                        Point {
                            x: width,
                            y: height,
                        },
                    ),
                };
                (start, end)
            }
        }
    }

    fn calculate_color(&self, pos: Point<Pixels>) -> Hsla {
        let x = pos.x;
        let y = pos.y;

        let t = match self.gradient_type {
            GradientType::Linear | GradientType::RepeatingLinear => {
                let dx = self.end.x - self.start.x;
                let dy = self.end.y - self.start.y;
                let dist = (dx * dx + dy * dy).0.sqrt();
                let dot = ((x - self.start.x) * dx + (y - self.start.y) * dy) / dist;
                dot / dist
            }
        };
        let t = if matches!(self.gradient_type, GradientType::RepeatingLinear) {
            t.0 % 1.0
        } else {
            t.0.clamp(0.0, 1.0)
        };
        let i = self
            .colors
            .iter()
            .enumerate()
            .take_while(|&(i, color_stop)| {
                t > color_stop.percentage.unwrap_or(1.0) && i < self.colors.len() - 2
            })
            .last()
            .map_or(0, |(i, _)| i);

        let start_percentage = self.colors[i].percentage.unwrap_or(0.0);
        let end_percentage = self
            .colors
            .get(i + 1)
            .map_or(1.0, |color_stop| color_stop.percentage.unwrap_or(1.0));

        let t = (t - start_percentage) / (end_percentage - start_percentage);
        self.colors[i]
            .color
            .interpolate(self.colors[i + 1].color, t)
    }
}
