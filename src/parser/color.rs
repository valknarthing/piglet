use anyhow::{Result, Context};
use csscolorparser::Color as CssColor;
use palette::rgb::Rgb;

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
        let color = CssColor::parse(hex)
            .context(format!("Failed to parse hex color: {}", hex))?;
        
        Ok(Self {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b