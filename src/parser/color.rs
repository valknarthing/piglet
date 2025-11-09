use anyhow::{Context, Result};
use csscolorparser::Color as CssColor;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let color = hex
            .parse::<CssColor>()
            .context(format!("Failed to parse hex color: {}", hex))?;

        Ok(Self {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        })
    }

    pub fn parse(color_str: &str) -> Result<Self> {
        Self::from_hex(color_str)
    }

    pub fn interpolate(&self, other: &Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 + (other.r as f64 - self.r as f64) * t) as u8,
            g: (self.g as f64 + (other.g as f64 - self.g as f64) * t) as u8,
            b: (self.b as f64 + (other.b as f64 - self.b as f64) * t) as u8,
        }
    }

    #[allow(dead_code)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}
