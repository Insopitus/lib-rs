use std::{
    iter::Sum,
    ops::{Add, AddAssign, DivAssign, Mul},
    str::FromStr,
};

/// Color for GPU rendering and game development
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}
pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

pub fn mix(c1: Color, c2: Color, alpha: f32) -> Color {
    c1 * (1.0 - alpha) + c2 * alpha
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    // fn from_rgba_array(arr: &[u8; 4]) -> Self {
    //     todo!()
    // }
    pub fn as_rgb_bytes(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }
    pub fn as_rgba8_bytes(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
    pub fn linear_to_gamma(&self, gamma: f32) -> Self {
        Self {
            r: self.r.powf(1.0 / gamma),
            g: self.g.powf(1.0 / gamma),
            b: self.b.powf(1.0 / gamma),
            a: self.a.powf(1.0 / gamma),
        }
    }
}
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
impl From<u32> for Color {
    fn from(_: u32) -> Self {
        todo!()
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.a += rhs.a;
        self.r += rhs.r;
        self.b += rhs.b;
        self.g += rhs.g;
    }
}
impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.a /= rhs;
        self.r /= rhs;
        self.b /= rhs;
        self.g /= rhs;
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(rgba(0.0, 0.0, 0.0, 0.0), |a, b| a + b)
    }
}

/// color names
impl Color {
    const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    const TRANSPARENT: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.0,
    };
}

#[cfg(test)]
mod test {
    use super::Color;

    fn basic() {
        Color::WHITE;
        "#ffff00".parse::<Color>();
        Color::BLACK * Color::WHITE;
    }
}
