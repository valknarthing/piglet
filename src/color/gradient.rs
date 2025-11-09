use crate::parser::gradient::Gradient;
use crate::parser::color::Color;
use anyhow::Result;

pub struct GradientEngine {
    gradient: Gradient,
}

impl GradientEngine {
    pub fn new(gradient: Gradient) -> Self {
        Self { gradient }
    }
    
    pub fn from_string(gradient_str: &str) -> Result<Self> {
        let gradient = Gradient::parse(gradient_str)?;
        Ok(Self::new(gradient))
    }
    
    pub fn color_at(&self, t: f64) -> Color {
        self.gradient.color_at(t)
    }
    
    pub fn colors(&self, steps: usize) -> Vec<Color> {
        self.gradient.colors(steps)
    }
}