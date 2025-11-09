pub mod apply;
pub mod gradient;
pub mod palette;

use crate::parser::color::Color;
use anyhow::Result;
pub use gradient::GradientEngine;
pub use palette::ColorPalette;

#[derive(Debug, Clone)]
pub enum ColorMode {
    None,
    Palette(ColorPalette),
    Gradient(GradientEngine),
}

pub struct ColorEngine {
    mode: ColorMode,
}

impl ColorEngine {
    pub fn new() -> Self {
        Self {
            mode: ColorMode::None,
        }
    }

    pub fn with_palette(mut self, palette: Option<&[String]>) -> Result<Self> {
        if let Some(colors) = palette {
            if !colors.is_empty() {
                let palette = ColorPalette::from_strings(colors)?;
                self.mode = ColorMode::Palette(palette);
            }
        }
        Ok(self)
    }

    pub fn with_gradient(mut self, gradient: Option<&str>) -> Result<Self> {
        if let Some(gradient_str) = gradient {
            let gradient = GradientEngine::from_string(gradient_str)?;
            self.mode = ColorMode::Gradient(gradient);
        }
        Ok(self)
    }

    pub fn has_colors(&self) -> bool {
        !matches!(self.mode, ColorMode::None)
    }

    #[allow(dead_code)]
    pub fn get_color(&self, t: f64, index: usize) -> Option<Color> {
        match &self.mode {
            ColorMode::None => None,
            ColorMode::Palette(palette) => Some(palette.get_color(index)),
            ColorMode::Gradient(gradient) => Some(gradient.color_at(t)),
        }
    }

    #[allow(dead_code)]
    pub fn get_colors(&self, steps: usize) -> Vec<Color> {
        match &self.mode {
            ColorMode::None => vec![],
            ColorMode::Palette(palette) => (0..steps).map(|i| palette.get_color(i)).collect(),
            ColorMode::Gradient(gradient) => gradient.colors(steps),
        }
    }

    pub fn color_at(&self, t: f64) -> Option<Color> {
        match &self.mode {
            ColorMode::None => None,
            ColorMode::Palette(palette) => {
                Some(palette.get_color((t * palette.len() as f64) as usize))
            }
            ColorMode::Gradient(gradient) => Some(gradient.color_at(t)),
        }
    }
}

impl Default for ColorEngine {
    fn default() -> Self {
        Self::new()
    }
}
