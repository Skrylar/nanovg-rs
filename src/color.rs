#[derive(Debug, Clone, Copy)]
/// Colors in NanoVG are stored as unsigned ints in ABGR format.
pub struct Color {
    pub a: f32,
    pub b: f32,
    pub g: f32,
    pub r: f32,
}

impl Color {
    /// Returns a color value from red, green, blue values. Alpha will be set to 255 (1.0f).
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color{
            r: (r as f32) / 255.0,
            g: (g as f32) / 255.0,
            b: (b as f32) / 255.0,
            a: 1.0,
        }
    }

    /// Returns a color value from red, green, blue values. Alpha will be set to 1.0f.
    pub fn rgbf(r: f32, g: f32, b: f32) -> Self {
        Color{
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }

    /// Returns a color value from red, green, blue and alpha values.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color{
            r: (r as f32) / 255.0,
            g: (g as f32) / 255.0,
            b: (b as f32) / 255.0,
            a: (a as f32) / 255.0,
        }
    }

    /// Returns a color value from red, green, blue and alpha values.
    pub fn rgbaf(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color{
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    /// Linearly interpolates from color c0 to c1, and returns resulting color value.
    pub fn lerp_rgba(c0: Color, c1: Color, u: f32) -> Self {
        unimplemented!();
    }

    /// Sets transparency of a color value.
    pub fn with_transparency(&self, a: u8) -> Self {
        Color{
            a: (a as f32) / 255.0,
            ..*self
        }
    }

    /// Sets transparency of a color value.
    pub fn with_transparencyf(&self, a: f32) -> Self {
        Color{
            a: a,
            ..*self
        }
    }

    /// Returns color value specified by hue, saturation and lightness.
    /// HSL values are all in range [0..1], alpha will be set to 255.
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        unimplemented!();
    }

    /// Returns color value specified by hue, saturation and lightness and alpha.
    /// HSL values are all in range [0..1], alpha in range [0..255]
    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        unimplemented!();
    }
}
