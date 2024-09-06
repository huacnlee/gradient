extern crate image;

use image::{ImageBuffer, Rgba};
use std::f32::consts::PI;
use std::fmt::Debug;

type Pixels = f32;

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

/// An HSLA color
#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Hsla {
    /// Hue, in a range from 0 to 1
    pub h: f32,

    /// Saturation, in a range from 0 to 1
    pub s: f32,

    /// Lightness, in a range from 0 to 1
    pub l: f32,

    /// Alpha, in a range from 0 to 1
    pub a: f32,
}

pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla {
        h: h.clamp(0., 1.),
        s: s.clamp(0., 1.),
        l: l.clamp(0., 1.),
        a: a.clamp(0., 1.),
    }
}

impl Hsla {
    fn interpolate(&self, other: &Hsla, t: Pixels) -> Hsla {
        let h = self.h * (1.0 - t) + other.h * t;
        let s = self.s * (1.0 - t) + other.s * t;
        let l = self.l * (1.0 - t) + other.l * t;
        let a = self.a * (1.0 - t) + other.a * t;
        hsla(h, s, l, a)
    }

    fn to_rgba(self) -> Rgba<u8> {
        let (r, g, b) = hsl_to_rgb(self.h, self.s, self.l);
        Rgba([r, g, b, (self.a * 255.0) as u8])
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

pub struct ColorStop {
    color: Hsla,
    percentage: Option<f32>,
}

fn color_stop(color: Hsla, percentage: Option<f32>) -> ColorStop {
    ColorStop { color, percentage }
}

pub enum GradientType {
    Linear,
    RepeatingLinear,
    Radial,
    Conic,
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum AngleOrCorner {
    Angle(f32),
    To(Side),
}

pub struct Gradient {
    colors: Vec<ColorStop>,
    gradient_type: GradientType,
    start: Point<Pixels>,
    end: Point<Pixels>,
}

impl Gradient {
    pub fn linear(
        angle_or_corner: AngleOrCorner,
        colors: Vec<ColorStop>,
        width: impl Into<Pixels>,
        height: impl Into<Pixels>,
    ) -> Gradient {
        let (start, end) =
            Gradient::calculate_start_end(angle_or_corner, width.into(), height.into());
        Gradient {
            colors,
            gradient_type: GradientType::Linear,
            start,
            end,
        }
    }

    pub fn repeating_linear(
        angle_or_corner: AngleOrCorner,
        colors: Vec<ColorStop>,
        width: impl Into<Pixels>,
        height: impl Into<Pixels>,
    ) -> Gradient {
        let (start, end) =
            Gradient::calculate_start_end(angle_or_corner, width.into(), height.into());
        Gradient {
            colors,
            gradient_type: GradientType::RepeatingLinear,
            start,
            end,
        }
    }

    pub fn radial(start: Point<Pixels>, colors: Vec<ColorStop>) -> Gradient {
        Gradient {
            colors,
            gradient_type: GradientType::Radial,
            start,
            end: Point { x: 1.0, y: 1.0 },
        }
    }

    pub fn conic(start: Point<Pixels>, colors: Vec<ColorStop>) -> Gradient {
        Gradient {
            colors,
            gradient_type: GradientType::Conic,
            start,
            end: Point { x: 1.0, y: 1.0 },
        }
    }

    fn calculate_start_end(
        angle_or_corner: AngleOrCorner,
        width: Pixels,
        height: Pixels,
    ) -> (Point<Pixels>, Point<Pixels>) {
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
            AngleOrCorner::To(side) => {
                let (start, end) = match side {
                    Side::Top => (
                        Point {
                            x: width / 2.0,
                            y: height,
                        },
                        Point {
                            x: width / 2.0,
                            y: 0.0,
                        },
                    ),
                    Side::Right => (
                        Point {
                            x: 0.0,
                            y: height / 2.0,
                        },
                        Point {
                            x: width,
                            y: height / 2.0,
                        },
                    ),
                    Side::Bottom => (
                        Point {
                            x: width / 2.0,
                            y: 0.0,
                        },
                        Point {
                            x: width / 2.0,
                            y: height,
                        },
                    ),
                    Side::Left => (
                        Point {
                            x: width,
                            y: height / 2.0,
                        },
                        Point {
                            x: 0.0,
                            y: height / 2.0,
                        },
                    ),
                    Side::TopLeft => (
                        Point {
                            x: width,
                            y: height,
                        },
                        Point { x: 0.0, y: 0.0 },
                    ),
                    Side::TopRight => (Point { x: 0.0, y: height }, Point { x: width, y: 0.0 }),
                    Side::BottomLeft => (Point { x: width, y: 0.0 }, Point { x: 0.0, y: height }),
                    Side::BottomRight => (
                        Point { x: 0.0, y: 0.0 },
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
                let dist = ((dx * dx + dy * dy) as Pixels).sqrt();
                let dot = ((x - self.start.x) * dx + (y - self.start.y) * dy) / dist;
                dot / dist
            }
            GradientType::Radial => {
                let cx = self.start.x;
                let cy = self.start.y;
                let max_dist = ((cx - self.end.x).powi(2) + (cy - self.end.y).powi(2)).sqrt();
                let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                dist / max_dist
            }
            GradientType::Conic => {
                let cx = self.start.x;
                let cy = self.start.y;

                ((y - cy).atan2(x - cx) + PI) / (2.0 * PI)
            }
        };

        let t = if matches!(self.gradient_type, GradientType::RepeatingLinear) {
            t % 1.0
        } else {
            t.clamp(0.0, 1.0)
        };

        let mut i = 0;
        while i < self.colors.len() - 1 && t > self.colors[i + 1].percentage.unwrap_or(1.0) {
            i += 1;
        }

        let start_percentage = self.colors[i].percentage.unwrap_or(0.0);
        let end_percentage = self.colors[i + 1].percentage.unwrap_or(1.0);

        let t = (t - start_percentage) / (end_percentage - start_percentage);
        self.colors[i]
            .color
            .interpolate(&self.colors[i + 1].color, t)
    }
}

fn generate_conic() {
    let width = 800;
    let height = 600;

    let gradient = Gradient::conic(
        Point {
            x: width as Pixels / 2.0,
            y: height as Pixels / 2.0,
        },
        vec![
            color_stop(hsla(0.0, 1.0, 0.5, 1.0), Some(0.0)), // red 0%
            color_stop(hsla(30.0 / 360.0, 1.0, 0.5, 1.0), Some(0.14)), // orange 14%
            color_stop(hsla(60.0 / 360.0, 1.0, 0.5, 1.0), Some(0.28)), // yellow 28%
            color_stop(hsla(120.0 / 360.0, 1.0, 0.5, 1.0), Some(0.42)), // green 42%
            color_stop(hsla(240.0 / 360.0, 1.0, 0.5, 1.0), Some(0.57)), // blue 57%
            color_stop(hsla(275.0 / 360.0, 1.0, 0.5, 1.0), Some(0.71)), // indigo 71%
            color_stop(hsla(300.0 / 360.0, 1.0, 0.5, 1.0), Some(0.85)), // violet 85%
            color_stop(hsla(0.0, 1.0, 0.5, 1.0), Some(1.0)), // red 100%
        ],
    );

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = gradient.calculate_color(Point {
            x: x as Pixels,
            y: y as Pixels,
        });
        *pixel = color.to_rgba();
    }

    img.save("gradient-conic.png").unwrap();
}

fn generate_linear() {
    let width = 800;
    let height = 600;

    let gradient = Gradient::linear(
        AngleOrCorner::To(Side::Right),
        vec![
            color_stop(hsla(0.0, 1.0, 0.5, 1.0), Some(0.0)), // red 0%
            color_stop(hsla(30.0 / 360.0, 1.0, 0.5, 1.0), Some(0.14)), // orange 14%
            color_stop(hsla(60.0 / 360.0, 1.0, 0.5, 1.0), Some(0.28)), // yellow 28%
            color_stop(hsla(120.0 / 360.0, 1.0, 0.5, 1.0), Some(0.42)), // green 42%
            color_stop(hsla(240.0 / 360.0, 1.0, 0.5, 1.0), Some(0.57)), // blue 57%
            color_stop(hsla(275.0 / 360.0, 1.0, 0.5, 1.0), Some(0.71)), // indigo 71%
            color_stop(hsla(300.0 / 360.0, 1.0, 0.5, 1.0), Some(0.85)), // violet 85%
            color_stop(hsla(0.0, 1.0, 0.5, 1.0), Some(1.0)), // red 100%
        ],
        width as Pixels,
        height as Pixels,
    );

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = gradient.calculate_color(Point {
            x: x as Pixels,
            y: y as Pixels,
        });
        *pixel = color.to_rgba();
    }

    img.save("gradient-linear.png").unwrap();
}

fn generate_radial() {
    let width = 800;
    let height = 600;

    let gradient = Gradient::radial(
        Point {
            x: width as Pixels / 2.0,
            y: height as Pixels / 2.0,
        },
        vec![
            color_stop(hsla(0.0, 1.0, 0.5, 1.0), Some(0.0)), // red 0%
            color_stop(hsla(0.0, 0.5, 1.0, 0.0), Some(0.5)), // white 100%
        ],
    );

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = gradient.calculate_color(Point {
            x: x as Pixels,
            y: y as Pixels,
        });
        *pixel = color.to_rgba();
    }

    img.save("radial_gradient.png").unwrap();
}

fn main() {
    generate_linear();
    generate_conic();
    generate_radial();
}
