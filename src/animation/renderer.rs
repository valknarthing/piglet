use crate::animation::{easing::EasingFunction, effects::Effect, timeline::Timeline};
use crate::color::{apply, ColorEngine};
use crate::utils::{ansi, ascii::AsciiArt, terminal::TerminalManager};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tokio::time::sleep;

pub struct Renderer<'a> {
    ascii_art: &'a AsciiArt,
    timeline: Timeline,
    effect: &'a dyn Effect,
    easing: &'a dyn EasingFunction,
    color_engine: &'a ColorEngine,
}

impl<'a> Renderer<'a> {
    pub fn new(
        ascii_art: &'a AsciiArt,
        duration_ms: u64,
        fps: u32,
        effect: &'a dyn Effect,
        easing: &'a dyn EasingFunction,
        color_engine: &'a ColorEngine,
    ) -> Self {
        Self {
            ascii_art,
            timeline: Timeline::new(duration_ms, fps),
            effect,
            easing,
            color_engine,
        }
    }

    pub async fn render(&self, terminal: &mut TerminalManager) -> Result<bool> {
        let mut timeline = Timeline::new(self.timeline.duration_ms(), self.timeline.fps());
        timeline.start();

        // Spawn background thread to listen for exit keys
        let should_exit = Arc::new(AtomicBool::new(false));
        let should_exit_clone = should_exit.clone();

        std::thread::spawn(move || {
            loop {
                if let Ok(true) = event::poll(Duration::from_millis(100)) {
                    if let Ok(Event::Key(key)) = event::read() {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                should_exit_clone.store(true, Ordering::Relaxed);
                                break;
                            }
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                should_exit_clone.store(true, Ordering::Relaxed);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                if should_exit_clone.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        loop {
            // Check for exit FIRST
            if should_exit.load(Ordering::Relaxed) {
                return Ok(true); // User requested exit
            }

            let frame_start = std::time::Instant::now();

            // Calculate progress with easing
            let linear_progress = timeline.progress();
            let eased_progress = self.easing.ease(linear_progress);

            // Check again before rendering
            if should_exit.load(Ordering::Relaxed) {
                return Ok(true); // User requested exit
            }

            // Apply effect
            let effect_result = self.effect.apply(self.ascii_art, eased_progress);

            // Apply colors if available
            let colored_text = if self.color_engine.has_colors() {
                self.apply_colors(&effect_result.text, linear_progress)
            } else {
                effect_result.text.clone()
            };

            // Check before terminal operations
            if should_exit.load(Ordering::Relaxed) {
                return Ok(true); // User requested exit
            }

            // Render to terminal
            terminal.clear()?;
            terminal.refresh_size()?;

            // Apply offsets and render
            if effect_result.offset_x == 0 && effect_result.offset_y == 0 {
                terminal.print_centered(&colored_text)?;
            } else {
                let (width, height) = terminal.get_size();
                let lines: Vec<&str> = colored_text.lines().collect();
                let text_height = lines.len() as i32;
                let text_width = lines
                    .iter()
                    .map(|l| ansi::visual_width(l))
                    .max()
                    .unwrap_or(0) as i32;

                let base_x = (width as i32 - text_width) / 2;
                let base_y = (height as i32 - text_height) / 2;

                let x = (base_x + effect_result.offset_x).max(0) as u16;
                let y = (base_y + effect_result.offset_y).max(0) as u16;

                for (i, line) in lines.iter().enumerate() {
                    let line_y = y.saturating_add(i as u16);
                    if line_y < height {
                        terminal.print_at(x, line_y, line)?;
                    }
                }
            }

            // Check if user wants to exit
            if should_exit.load(Ordering::Relaxed) {
                return Ok(true); // User requested exit
            }

            // Check if animation is complete before advancing
            if timeline.is_complete() {
                return Ok(false); // Animation completed naturally
            }

            // Advance to next frame and wait
            timeline.next_frame();
            let frame_duration = timeline.frame_duration();
            let elapsed = frame_start.elapsed();

            if elapsed < frame_duration {
                let sleep_duration = frame_duration - elapsed;
                // Break sleep into small chunks to check should_exit frequently
                let chunk_duration = Duration::from_millis(5);
                let mut remaining = sleep_duration;

                while remaining > Duration::ZERO {
                    if should_exit.load(Ordering::Relaxed) {
                        return Ok(true); // User requested exit during sleep
                    }
                    let sleep_time = remaining.min(chunk_duration);
                    sleep(sleep_time).await;
                    remaining = remaining.saturating_sub(sleep_time);
                }
            }
        }
    }

    fn apply_colors(&self, text: &str, progress: f64) -> String {
        match self.effect.name() {
            "rainbow" | "color-cycle" => {
                // For rainbow/color-cycle effects, use gradient across characters
                let char_count = text.chars().filter(|c| !c.is_whitespace()).count();
                let colors = self.color_engine.get_colors(char_count);
                apply::apply_gradient_to_text(text, &colors)
            }
            "gradient-flow" => {
                // For gradient-flow, shift colors based on progress
                let char_count = text.chars().filter(|c| !c.is_whitespace()).count();
                let mut colors = self.color_engine.get_colors(char_count * 2);
                let offset = (progress * colors.len() as f64) as usize;
                let len = colors.len();
                colors.rotate_left(offset % len);
                colors.truncate(char_count);
                apply::apply_gradient_to_text(text, &colors)
            }
            _ => {
                // For other effects, use gradient based on progress
                if let Some(color) = self.color_engine.color_at(progress) {
                    let lines: Vec<String> = text
                        .lines()
                        .map(|line| apply::apply_color_to_line(line, &[color]))
                        .collect();
                    lines.join("\n")
                } else {
                    let char_count = text.chars().filter(|c| !c.is_whitespace()).count();
                    let colors = self.color_engine.get_colors(char_count.max(10));
                    apply::apply_gradient_to_text(text, &colors)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::easing::Linear;
    use crate::animation::effects::FadeIn;

    #[test]
    fn test_renderer_creation() {
        let ascii_art = AsciiArt::new("Test".to_string());
        let effect = FadeIn;
        let easing = Linear;
        let color_engine = ColorEngine::new();

        let renderer = Renderer::new(&ascii_art, 1000, 30, &effect, &easing, &color_engine);

        assert_eq!(renderer.timeline.duration_ms(), 1000);
        assert_eq!(renderer.timeline.fps(), 30);
    }
}
