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

// Phase 2: Specialty & Combination Effects

// Puff-in effect - scale up from tiny with fade in
pub struct PuffIn;
impl Effect for PuffIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Start very small and expand while fading in
        let scale = 0.1 + (progress * 0.9);
        let opacity = progress;
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "puff-in"
    }
}

// Puff-out effect - scale down to tiny with fade out
pub struct PuffOut;
impl Effect for PuffOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Shrink down while fading out
        let scale = 1.0 - (progress * 0.9);
        let opacity = 1.0 - progress;
        let scaled = ascii_art.scale(scale.max(0.1));
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "puff-out"
    }
}

// Slide-rotate horizontal - slide from left with rotation
pub struct SlideRotateHor;
impl Effect for SlideRotateHor {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Slide in from left while rotating
        let offset_x = ((1.0 - progress) * -(ascii_art.width() as f64 + 10.0)) as i32;
        let rotation_progress = 1.0 - progress;
        let offset_y =
            (rotation_progress * 10.0 * (rotation_progress * std::f64::consts::PI).sin()) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "slide-rotate-hor"
    }
}

// Slide-rotate vertical - slide from top with rotation
pub struct SlideRotateVer;
impl Effect for SlideRotateVer {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Slide in from top while rotating
        let offset_y = ((1.0 - progress) * -(ascii_art.height() as f64 + 5.0)) as i32;
        let rotation_progress = 1.0 - progress;
        let offset_x =
            (rotation_progress * 15.0 * (rotation_progress * std::f64::consts::PI).cos()) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "slide-rotate-ver"
    }
}

// Flicker effect - random flickering opacity
pub struct Flicker;
impl Effect for Flicker {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Fast flickering that stabilizes
        let flicker_speed = 30.0;
        let stability = progress; // Gets more stable over time
        let flicker = ((progress * flicker_speed).sin() + 1.0) / 2.0;
        let opacity = stability + (1.0 - stability) * flicker;
        EffectResult::new(ascii_art.render()).with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "flicker"
    }
}

// Tracking-in effect - letters expand from center
pub struct TrackingIn;
impl Effect for TrackingIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Simulate letter spacing by adding spaces between characters
        let spacing = ((1.0 - progress) * 3.0) as usize;
        if spacing == 0 {
            EffectResult::new(ascii_art.render())
        } else {
            let lines: Vec<String> = ascii_art
                .get_lines()
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if c == ' ' {
                                " ".repeat(spacing + 1)
                            } else {
                                format!("{}{}", c, " ".repeat(spacing))
                            }
                        })
                        .collect::<String>()
                })
                .collect();
            EffectResult::new(lines.join("\n"))
        }
    }

    fn name(&self) -> &str {
        "tracking-in"
    }
}

// Tracking-out effect - letters contract to center
pub struct TrackingOut;
impl Effect for TrackingOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Simulate letter spacing by adding spaces between characters
        let spacing = (progress * 3.0) as usize;
        if spacing == 0 {
            EffectResult::new(ascii_art.render())
        } else {
            let lines: Vec<String> = ascii_art
                .get_lines()
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if c == ' ' {
                                " ".repeat(spacing + 1)
                            } else {
                                format!("{}{}", c, " ".repeat(spacing))
                            }
                        })
                        .collect::<String>()
                })
                .collect();
            EffectResult::new(lines.join("\n"))
        }
    }

    fn name(&self) -> &str {
        "tracking-out"
    }
}

// Bounce-top effect - bounce down from top
pub struct BounceTop;
impl Effect for BounceTop {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Bounce from top with easing
        let bounces = 2.0;
        let bounce_height = ascii_art.height() as f64 + 10.0;
        let base_offset = (1.0 - progress) * bounce_height;
        let bounce_factor =
            (progress * bounces * std::f64::consts::PI).sin().abs() * (1.0 - progress);
        let offset_y = -(base_offset + bounce_factor * 5.0) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "bounce-top"
    }
}

// Bounce-bottom effect - bounce up from bottom
pub struct BounceBottom;
impl Effect for BounceBottom {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Bounce from bottom with easing
        let bounces = 2.0;
        let bounce_height = ascii_art.height() as f64 + 10.0;
        let base_offset = (1.0 - progress) * bounce_height;
        let bounce_factor =
            (progress * bounces * std::f64::consts::PI).sin().abs() * (1.0 - progress);
        let offset_y = (base_offset + bounce_factor * 5.0) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "bounce-bottom"
    }
}

// Tilt-in effect - tilt in with perspective simulation
pub struct TiltIn;
impl Effect for TiltIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Simulate tilting in with combined scale and offset
        let tilt_progress = 1.0 - progress;
        let scale = 0.5 + (progress * 0.5);
        let offset_x = (tilt_progress * 20.0 * (tilt_progress * std::f64::consts::PI).sin()) as i32;
        let offset_y = -(tilt_progress * 15.0) as i32;
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_offset(offset_x, offset_y)
    }

    fn name(&self) -> &str {
        "tilt-in"
    }
}

// Phase 3: Additional Slide, Blink, Focus, and Shadow Effects

// Slide-out-top effect - slide out to top
pub struct SlideOutTop;
impl Effect for SlideOutTop {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = -(progress * (ascii_art.height() as f64 + 10.0)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "slide-out-top"
    }
}

// Slide-out-bottom effect - slide out to bottom
pub struct SlideOutBottom;
impl Effect for SlideOutBottom {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_y = (progress * (ascii_art.height() as f64 + 10.0)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(0, offset_y)
    }

    fn name(&self) -> &str {
        "slide-out-bottom"
    }
}

// Slide-out-left effect - slide out to left
pub struct SlideOutLeft;
impl Effect for SlideOutLeft {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_x = -(progress * (ascii_art.width() as f64 + 10.0)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "slide-out-left"
    }
}

// Slide-out-right effect - slide out to right
pub struct SlideOutRight;
impl Effect for SlideOutRight {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        let offset_x = (progress * (ascii_art.width() as f64 + 10.0)) as i32;
        EffectResult::new(ascii_art.render()).with_offset(offset_x, 0)
    }

    fn name(&self) -> &str {
        "slide-out-right"
    }
}

// Blink effect - rapid on/off blinking
pub struct Blink;
impl Effect for Blink {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Blink 3 times during animation
        let blinks = 6.0;
        let blink_state = ((progress * blinks).floor() % 2.0) as i32;
        let opacity = if blink_state == 0 { 1.0 } else { 0.0 };
        EffectResult::new(ascii_art.render()).with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "blink"
    }
}

// Focus-in effect - simulate coming into focus with scale and opacity
pub struct FocusIn;
impl Effect for FocusIn {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Start blurry (small scale, low opacity) and come into focus
        let scale = 0.7 + (progress * 0.3);
        let opacity = progress.powf(0.5);
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "focus-in"
    }
}

// Blur-out effect - simulate going out of focus
pub struct BlurOut;
impl Effect for BlurOut {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Go out of focus (reduce scale, reduce opacity)
        let scale = 1.0 - (progress * 0.3);
        let opacity = (1.0 - progress).powf(0.5);
        let scaled = ascii_art.scale(scale);
        EffectResult::new(scaled.render())
            .with_scale(scale)
            .with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "blur-out"
    }
}

// Shadow-drop effect - drop down with shadow simulation
pub struct ShadowDrop;
impl Effect for ShadowDrop {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Drop down from above with increasing shadow (opacity)
        let drop_distance = 20.0;
        let offset_y = -((1.0 - progress) * drop_distance) as i32;
        let opacity = 0.3 + (progress * 0.7); // Start semi-transparent
        EffectResult::new(ascii_art.render())
            .with_offset(0, offset_y)
            .with_opacity(opacity)
    }

    fn name(&self) -> &str {
        "shadow-drop"
    }
}

// Shadow-pop effect - pop forward with shadow simulation
pub struct ShadowPop;
impl Effect for ShadowPop {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Scale up quickly then settle, simulating popping forward
        let pop_scale = if progress < 0.5 {
            1.0 + (progress * 2.0) * 0.3
        } else {
            1.3 - ((progress - 0.5) * 2.0) * 0.3
        };
        let scaled = ascii_art.scale(pop_scale);
        EffectResult::new(scaled.render()).with_scale(pop_scale)
    }

    fn name(&self) -> &str {
        "shadow-pop"
    }
}

// Rotate-center effect - rotate around center point
pub struct RotateCenter;
impl Effect for RotateCenter {
    fn apply(&self, ascii_art: &AsciiArt, progress: f64) -> EffectResult {
        // Simulate rotation with alternating line offsets
        let rotations = 1.0;
        let angle = progress * rotations * std::f64::consts::PI * 2.0;
        let max_offset = 5.0;

        let lines: Vec<String> = ascii_art
            .get_lines()
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let line_factor = (i as f64 / ascii_art.get_lines().len().max(1) as f64) - 0.5;
                let offset = (angle.sin() * line_factor * max_offset) as i32;
                if offset > 0 {
                    format!("{}{}", " ".repeat(offset as usize), line)
                } else if offset < 0 {
                    line.chars().skip(offset.unsigned_abs() as usize).collect()
                } else {
                    line.to_string()
                }
            })
            .collect();

        EffectResult::new(lines.join("\n"))
    }

    fn name(&self) -> &str {
        "rotate-center"
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
        "puff-in" => Ok(Box::new(PuffIn)),
        "puff-out" => Ok(Box::new(PuffOut)),
        "slide-rotate-hor" => Ok(Box::new(SlideRotateHor)),
        "slide-rotate-ver" => Ok(Box::new(SlideRotateVer)),
        "flicker" => Ok(Box::new(Flicker)),
        "tracking-in" => Ok(Box::new(TrackingIn)),
        "tracking-out" => Ok(Box::new(TrackingOut)),
        "bounce-top" => Ok(Box::new(BounceTop)),
        "bounce-bottom" => Ok(Box::new(BounceBottom)),
        "tilt-in" => Ok(Box::new(TiltIn)),
        "slide-out-top" => Ok(Box::new(SlideOutTop)),
        "slide-out-bottom" => Ok(Box::new(SlideOutBottom)),
        "slide-out-left" => Ok(Box::new(SlideOutLeft)),
        "slide-out-right" => Ok(Box::new(SlideOutRight)),
        "blink" => Ok(Box::new(Blink)),
        "focus-in" => Ok(Box::new(FocusIn)),
        "blur-out" => Ok(Box::new(BlurOut)),
        "shadow-drop" => Ok(Box::new(ShadowDrop)),
        "shadow-pop" => Ok(Box::new(ShadowPop)),
        "rotate-center" => Ok(Box::new(RotateCenter)),
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
        "puff-in",
        "puff-out",
        "slide-rotate-hor",
        "slide-rotate-ver",
        "flicker",
        "tracking-in",
        "tracking-out",
        "bounce-top",
        "bounce-bottom",
        "tilt-in",
        "slide-out-top",
        "slide-out-bottom",
        "slide-out-left",
        "slide-out-right",
        "blink",
        "focus-in",
        "blur-out",
        "shadow-drop",
        "shadow-pop",
        "rotate-center",
    ]
}
