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

// Phase 1: High-Impact Effects from Animista

// Shake effect - horizontal vibration
pub struct Shake;
impl Effect for Shake {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Fast oscillation that decreases over time
        let frequency = 20.0;
        let amplitude = 10.0 * (1.0 - progress);
        let offset_x = (progress * frequency * std::f64::consts::PI * 2.0).sin() * amplitude;
        EffectResult::new(ascii_art.render()).with_offset(offset_x as i32, 0)
    }

    fn name(&self) -> &str {
        "shake"
    }
}

// Wobble effect - rotation wobble (simulated with offset variations)
pub struct Wobble;
impl Effect for Wobble {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Wobble with decreasing amplitude
        let angle = progress * std::f64::consts::PI * 4.0;
        let amplitude = 15.0 * (1.0 - progress);
        let offset_x = (angle.sin() * amplitude) as i32;
        let offset_y = (angle.cos() * amplitude * 0.3) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "wobble"
    }
}

// Vibrate effect - rapid small movements
pub struct Vibrate;
impl Effect for Vibrate {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Very fast, small vibrations
        let frequency = 50.0;
        let amplitude = 3.0;
        let offset_x = (progress * frequency * std::f64::consts::PI).sin() * amplitude;
        let offset_y = (progress * frequency * std::f64::consts::PI * 1.3).cos() * amplitude;
        EffectResult::new(ascii_art.render()).with_offset(offset_x as i32, offset_y as i32)
    }

    fn name(&self) -> &str {
        "vibrate"
    }
}

// Heartbeat effect - pulsing scale with heartbeat rhythm
pub struct Heartbeat;
impl Effect for Heartbeat {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Two-beat pulse pattern like a heartbeat
        let beat_progress = (progress * 2.0) % 1.0;
        let scale = if beat_progress < 0.3 {
            1.0 + (beat_progress / 0.3) * 0.15
        } else if beat_progress < 0.4 {
            1.15 - ((beat_progress - 0.3) / 0.1) * 0.15
        } else if beat_progress < 0.6 {
            1.0 + ((beat_progress - 0.4) / 0.2) * 0.1
        } else if beat_progress < 0.7 {
            1.1 - ((beat_progress - 0.6) / 0.1) * 0.1
        } else {
            1.0
        };
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render()).with_scale(scale)
    }

    fn name(&self) -> &str {
        "heartbeat"
    }
}

// Flip horizontal - flip text horizontally
pub struct FlipHorizontal;
impl Effect for FlipHorizontal {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Scale horizontally from 1 to -1 (flip)
        let scale = 1.0 - (progress * 2.0);
        if scale <= 0.0 {
            // Show reversed text when flipped
            let lines: Vec<String> = ascii_art
                .get_lines()
                .iter()
                .map(|line| line.chars().rev().collect())
                .collect();
            EffectResult::new(lines.join("\n"))
        } else {
            let scaled = ascii_art.scale(scale);
            EffectResult::new(scaled.render()).with_scale(scale)
        }
    }

    fn name(&self) -> &str {
        "flip-horizontal"
    }
}

// Flip vertical - flip text vertically
pub struct FlipVertical;
impl Effect for FlipVertical {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Scale vertically with midpoint flip
        let scale = 1.0 - (progress * 2.0).min(1.0);
        if progress > 0.5 {
            // Show reversed lines when flipped
            let mut lines: Vec<String> = ascii_art
                .get_lines()
                .iter()
                .map(|s| s.to_string())
                .collect();
            lines.reverse();
            let result_scale = (progress - 0.5) * 2.0;
            let scaled = AsciiArt::new(lines.join("\n")).scale(result_scale);
            EffectResult::new(scaled.render()).with_scale(result_scale)
        } else {
            let scaled = ascii_art.scale(scale.max(0.1));
            EffectResult::new(scaled.render()).with_scale(scale.max(0.1))
        }
    }

    fn name(&self) -> &str {
        "flip-vertical"
    }
}

// Swing effect - pendulum motion
pub struct Swing;
impl Effect for Swing {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Pendulum swing with decreasing amplitude
        let swings = 2.0;
        let angle = (progress * swings * std::f64::consts::PI * 2.0).sin() * (1.0 - progress);
        let offset_x = (angle * 20.0) as i32;
        let offset_y = (angle.abs() * 5.0) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, -offset_y)
    }

    fn name(&self) -> &str {
        "swing"
    }
}

// Sway effect - gentle swaying motion
pub struct Sway;
impl Effect for Sway {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Smooth, gentle sway
        let angle = (progress * std::f64::consts::PI * 2.0).sin();
        let offset_x = (angle * 8.0) as i32;
        let offset_y = (angle.abs() * 2.0) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "sway"
    }
}

// Roll-in effect - roll in from left with rotation
pub struct RollIn;
impl Effect for RollIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Slide in from left while appearing to roll
        let offset_x = ((1.0 - progress) * -(ascii_art.width() as f64 + 20.0)) as i32;
        let rotation_effect = ((1.0 - progress) * 5.0) as i32;
        let offset_y = (rotation_effect as f64 * (progress * std::f64::consts::PI).sin()) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "roll-in"
    }
}

// Roll-out effect - roll out to right with rotation
pub struct RollOut;
impl Effect for RollOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Slide out to right while appearing to roll
        let offset_x = (progress * (ascii_art.width() as f64 + 20.0)) as i32;
        let rotation_effect = (progress * 5.0) as i32;
        let offset_y = (rotation_effect as f64 * (progress * std::f64::consts::PI).sin()) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "roll-out"
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
        "shake" => Ok(Box::new(Shake)),
        "wobble" => Ok(Box::new(Wobble)),
        "vibrate" => Ok(Box::new(Vibrate)),
        "heartbeat" => Ok(Box::new(Heartbeat)),
        "flip-horizontal" => Ok(Box::new(FlipHorizontal)),
        "flip-vertical" => Ok(Box::new(FlipVertical)),
        "swing" => Ok(Box::new(Swing)),
        "sway" => Ok(Box::new(Sway)),
        "roll-in" => Ok(Box::new(RollIn)),
        "roll-out" => Ok(Box::new(RollOut)),
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
        "shake",
        "wobble",
        "vibrate",
        "heartbeat",
        "flip-horizontal",
        "flip-vertical",
        "swing",
        "sway",
        "roll-in",
        "roll-out",
    ]
}
