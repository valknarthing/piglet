use crate::parser::color::Color;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ColorPalette {
    colors: Vec<Color>,
}

impl ColorPalette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    pub fn from_strings(color_strs: &[String]) -> Result<Self> {
        let colors: Result<Vec<Color>> = color_strs.iter().map(|s| Color::parse(s)).collect();
        Ok(Self::new(colors?))
    }

    pub fn get_color(&self, index: usize) -> Color {
        if self.colors.is_empty() {
            return Color::new(255, 255, 255);
        }
        self.colors[index % self.colors.len()]
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Create rainbow palette
    pub fn rainbow() -> Self {
        Self::from_strings(&[
            "#ff0000".to_string(),
            "#ff7f00".to_string(),
            "#ffff00".to_string(),
            "#00ff00".to_string(),
            "#0000ff".to_string(),
            "#4b0082".to_string(),
            "#9400d3".to_string(),
        ])
        .unwrap()
    }

    /// Create ocean palette
    #[allow(dead_code)]
    pub fn ocean() -> Self {
        Self::from_strings(&[
            "#000080".to_string(),
            "#0000ff".to_string(),
            "#4169e1".to_string(),
            "#87ceeb".to_string(),
            "#add8e6".to_string(),
        ])
        .unwrap()
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::rainbow()
    }
}
