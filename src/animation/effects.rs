use crate::utils::ascii::AsciiArt;
use anyhow::{bail, Result};

pub trait Effect: Send + Sync {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct EffectResult {
    pub text: String,
    pub opacity: f64,
    pub offset_x: i32,
    pub offset_y: i32,
    pub scale: f64,
}

impl EffectResult {
    pub fn new(text: String) -> Self {
        Self {
            text,
            opacity: 1.0,
            offset_x: 0,
            offset_y: 0,
            scale: 1.0,
        }
    }

    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_offset(mut self, x: i32, y: i32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}

// Fade effects
pub struct FadeIn;
impl Effect for FadeIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let text = ascii_art.apply_fade(progress);
        EffectResult::new(text).with_opacity(progress)
    }

    fn name(&self) -> &str {
        "fade-in"
    }
}

pub struct FadeOut;
impl Effect for FadeOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let opacity = 1.0 - progress;
        let text = ascii_art.apply_fade(opacity);
        EffectResult::new(text).with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "fade-out"
    }
}

pub struct FadeInOut;
impl Effect for FadeInOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let opacity = if progress < 0.5 {
            progress * 2.0
        } else {
            (1.0 - progress) * 2.0
        };
        let text = ascii_art.apply_fade(opacity);
        EffectResult::new(text).with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "fade-in-out"
    }
}

// Slide effects
pub struct SlideInTop;
impl Effect for SlideInTop {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = ((1.0 - progress) * -(ascii_art.height() as f64)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "slide-in-top"
    }
}

pub struct SlideInBottom;
impl Effect for SlideInBottom {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = ((1.0 - progress) * ascii_art.height() as f64) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "slide-in-bottom"
    }
}

pub struct SlideInLeft;
impl Effect for SlideInLeft {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_x = ((1.0 - progress) * -(ascii_art.width() as f64)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "slide-in-left"
    }
}

pub struct SlideInRight;
impl Effect for SlideInRight {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_x = ((1.0 - progress) * ascii_art.width() as f64) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "slide-in-right"
    }
}

// Scale effects
pub struct ScaleUp;
impl Effect for ScaleUp {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let scale = progress;
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render()).with_scale(scale)
    }

    fn name(&self) -> &str {
        "scale-up"
    }
}

pub struct ScaleDown;
impl Effect for ScaleDown {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let scale = 2.0 - progress;
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render()).with_scale(scale)
    }

    fn name(&self) -> &str {
        "scale-down"
    }
}

// Pulse effect
pub struct Pulse;
impl Effect for Pulse {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let scale = 1.0 + (progress * std::f64::consts::PI * 2.0).sin() * 0.1;
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render()).with_scale(scale)
    }

    fn name(&self) -> &str {
        "pulse"
    }
}

// Bounce effects
pub struct BounceIn;
impl Effect for BounceIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = if progress < 0.8 {
            ((1.0 - progress / 0.8) * -(ascii_art.height() as f64)) as i32
        } else {
            let bounce_progress = (progress - 0.8) / 0.2;
            (bounce_progress * 10.0 * (1.0 - bounce_progress)) as i32
        };
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "bounce-in"
    }
}

pub struct BounceOut;
impl Effect for BounceOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = if progress < 0.2 {
            -(progress * 10.0 * (1.0 - progress / 0.2)) as i32
        } else {
            (((progress - 0.2) / 0.8) * ascii_art.height() as f64) as i32
        };
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "bounce-out"
    }
}

// Typewriter effect
pub struct Typewriter;
impl Effect for Typewriter {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let total_chars = ascii_art.char_count();
        let visible_chars = (total_chars as f64 * progress) as usize;

        let positions = ascii_art.char_positions();
        let lines = ascii_art.get_lines();
        let mut result_lines: Vec<String> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| if c.is_whitespace() { c } else { ' ' })
                    .collect()
            })
            .collect();

        for (i, (x, y, ch)) in positions.iter().enumerate() {
            if i < visible_chars {
                if let Some(line) = result_lines.get_mut(*y) {
                    let mut chars: Vec<char> = line.chars().collect();
                    if *x < chars.len() {
                        chars[*x] = *ch;
                        *line = chars.iter().collect();
                    }
                }
            }
        }

        EffectResult::new(result_lines.join("\n"))
    }

    fn name(&self) -> &str {
        "typewriter"
    }
}

pub struct TypewriterReverse;
impl Effect for TypewriterReverse {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let total_chars = ascii_art.char_count();
        let visible_chars = (total_chars as f64 * (1.0 - progress)) as usize;

        let positions = ascii_art.char_positions();
        let lines = ascii_art.get_lines();
        let mut result_lines: Vec<String> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| if c.is_whitespace() { c } else { ' ' })
                    .collect()
            })
            .collect();

        for (i, (x, y, ch)) in positions.iter().enumerate() {
            if i < visible_chars {
                if let Some(line) = result_lines.get_mut(*y) {
                    let mut chars: Vec<char> = line.chars().collect();
                    if *x < chars.len() {
                        chars[*x] = *ch;
                        *line = chars.iter().collect();
                    }
                }
            }
        }

        EffectResult::new(result_lines.join("\n"))
    }

    fn name(&self) -> &str {
        "typewriter-reverse"
    }
}

// Wave effect
pub struct Wave;
impl Effect for Wave {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let lines: Vec<String> = ascii_art
            .get_lines()
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let wave_offset =
                    ((progress * std::f64::consts::PI * 2.0 + i as f64 * 0.5).sin() * 3.0) as usize;
                format!("{}{}", " ".repeat(wave_offset), line)
            })
            .collect();

        EffectResult::new(lines.join("\n"))
    }

    fn name(&self) -> &str {
        "wave"
    }
}

// Jello effect
pub struct Jello;
impl Effect for Jello {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let wobble = (progress * std::f64::consts::PI * 4.0).sin() * (1.0 - progress);
        let scale = 1.0 + wobble * 0.1;
        let scaled = ascii_art.scale(scale.abs());
        EffectResult::new(scaled.render()).with_scale(scale.abs())
    }

    fn name(&self) -> &str {
        "jello"
    }
}

// Rotate effects
pub struct RotateIn;
impl Effect for RotateIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Simulate rotation with scale and offset
        let angle = (1.0 - progress) * std::f64::consts::PI;
        let scale = progress;
        let scaled = ascii_art.scale(scale);
        let offset_x = (angle.cos() * 10.0 * (1.0 - progress)) as i32;
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "rotate-in"
    }
}

pub struct RotateOut;
impl Effect for RotateOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let angle = progress * std::f64::consts::PI;
        let scale = 1.0 - progress;
        let scaled = ascii_art.scale(scale);
        let offset_x = (angle.cos() * 10.0 * progress) as i32;
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "rotate-out"
    }
}

// Color effects (these will be enhanced by color engine)
pub struct ColorCycle;
impl Effect for ColorCycle {
    fn apply(&self, ascii_art: &AsciiArt, _progress: f64) -> EffectResult {
        EffectResult::new(ascii_art.render())
    }

    fn name(&self) -> &str {
        "color-cycle"
    }
}

pub struct Rainbow;
impl Effect for Rainbow {
    fn apply(&self, ascii_art: &AsciiArt, _progress: f64) -> EffectResult {
        EffectResult::new(ascii_art.render())
    }

    fn name(&self) -> &str {
        "rainbow"
    }
}

pub struct GradientFlow;
impl Effect for GradientFlow {
    fn apply(&self, ascii_art: &AsciiArt, _progress: f64) -> EffectResult {
        EffectResult::new(ascii_art.render())
    }

    fn name(&self) -> &str {
        "gradient-flow"
    }
}

/// Get effect by name
pub fn get_effect(name: &str) -> Result<Box<dyn Effect>> {
    match name {
        "fade-in" => Ok(Box::new(FadeIn)),
        "fade-out" => Ok(Box::new(FadeOut)),
        "fade-in-out" => Ok(Box::new(FadeInOut)),
        "slide-in-top" => Ok(Box::new(SlideInTop)),
        "slide-in-bottom" => Ok(Box::new(SlideInBottom)),
        "slide-in-left" => Ok(Box::new(SlideInLeft)),
        "slide-in-right" => Ok(Box::new(SlideInRight)),
        "scale-up" => Ok(Box::new(ScaleUp)),
        "scale-down" => Ok(Box::new(ScaleDown)),
        "pulse" => Ok(Box::new(Pulse)),
        "bounce-in" => Ok(Box::new(BounceIn)),
        "bounce-out" => Ok(Box::new(BounceOut)),
        "typewriter" => Ok(Box::new(Typewriter)),
        "typewriter-reverse" => Ok(Box::new(TypewriterReverse)),
        "wave" => Ok(Box::new(Wave)),
        "jello" => Ok(Box::new(Jello)),
        "color-cycle" => Ok(Box::new(ColorCycle)),
        "rainbow" => Ok(Box::new(Rainbow)),
        "gradient-flow" => Ok(Box::new(GradientFlow)),
        "rotate-in" => Ok(Box::new(RotateIn)),
        "rotate-out" => Ok(Box::new(RotateOut)),
        _ => bail!("Unknown effect: {}", name),
    }
}

/// List all available effects
#[allow(dead_code)]
pub fn list_effects() -> Vec<&'static str> {
    vec![
        "fade-in",
        "fade-out",
        "fade-in-out",
        "slide-in-top",
        "slide-in-bottom",
        "slide-in-left",
        "slide-in-right",
        "scale-up",
        "scale-down",
        "pulse",
        "bounce-in",
        "bounce-out",
        "typewriter",
        "typewriter-reverse",
        "wave",
        "jello",
        "color-cycle",
        "rainbow",
        "gradient-flow",
        "rotate-in",
        "rotate-out",
    ]
}
