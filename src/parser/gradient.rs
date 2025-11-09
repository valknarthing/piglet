use crate::parser::color::Color;
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub color: Color,
    pub position: f64,
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub stops: Vec<ColorStop>,
    #[allow(dead_code)] pub angle: f64,
}

impl Gradient {
    pub fn new(stops: Vec<ColorStop>, angle: f64) -> Self {
        Self { stops, angle }
    }

    pub fn parse(gradient_str: &str) -> Result<Self> {
        let gradient_str = gradient_str.trim();

        if !gradient_str.starts_with("linear-gradient(") {
            bail!("Only linear-gradient is supported");
        }

        let content = gradient_str
            .strip_prefix("linear-gradient(")
            .and_then(|s| s.strip_suffix(")"))
            .ok_or_else(|| anyhow::anyhow!("Invalid gradient syntax"))?;

        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        if parts.is_empty() {
            bail!("Gradient must have at least one color");
        }

        let mut angle = 180.0;
        let mut color_parts = parts.as_slice();

        if let Some(first) = parts.first() {
            if first.ends_with("deg") {
                angle = first
                    .trim_end_matches("deg")
                    .trim()
                    .parse()
                    .unwrap_or(180.0);
                color_parts = &parts[1..];
            } else if first.starts_with("to ") {
                angle = match first.trim() {
                    "to right" => 90.0,
                    "to left" => 270.0,
                    "to top" => 0.0,
                    "to bottom" => 180.0,
                    _ => 180.0,
                };
                color_parts = &parts[1..];
            }
        }

        let mut stops = Vec::new();
        let count = color_parts.len();

        for (i, part) in color_parts.iter().enumerate() {
            let part = part.trim();
            let mut color_str = part;
            let mut position = i as f64 / (count - 1).max(1) as f64;

            // Check if there's a percentage (e.g., "#FF5733 50%" or "red 50%")
            if let Some(percent_pos) = part.rfind('%') {
                // Find the last space before the percentage
                if let Some(space_pos) = part[..percent_pos].rfind(|c: char| c.is_whitespace()) {
                    color_str = part[..space_pos].trim();
                    let percent_str = part[space_pos + 1..percent_pos].trim();
                    if let Ok(p) = percent_str.parse::<f64>() {
                        position = p / 100.0;
                    }
                }
            }

            let color = Color::parse(color_str)?;
            stops.push(ColorStop { color, position });
        }

        Ok(Self::new(stops, angle))
    }

    pub fn color_at(&self, t: f64) -> Color {
        if self.stops.is_empty() {
            return Color::new(255, 255, 255);
        }

        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        let t = t.clamp(0.0, 1.0);

        for i in 0..self.stops.len() - 1 {
            let stop1 = &self.stops[i];
            let stop2 = &self.stops[i + 1];

            if t >= stop1.position && t <= stop2.position {
                let local_t = (t - stop1.position) / (stop2.position - stop1.position);
                return stop1.color.interpolate(&stop2.color, local_t);
            }
        }

        self.stops.last().unwrap().color
    }

    pub fn colors(&self, steps: usize) -> Vec<Color> {
        (0..steps)
            .map(|i| {
                let t = i as f64 / (steps - 1).max(1) as f64;
                self.color_at(t)
            })
            .collect()
    }
}
