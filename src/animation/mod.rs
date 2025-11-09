pub mod easing;
pub mod effects;
pub mod renderer;
pub mod timeline;

use crate::color::ColorEngine;
use crate::utils::{ascii::AsciiArt, terminal::TerminalManager};
use anyhow::Result;

pub struct AnimationEngine {
    ascii_art: AsciiArt,
    duration_ms: u64,
    fps: u32,
    effect: Box<dyn effects::Effect>,
    easing: Box<dyn easing::EasingFunction>,
    color_engine: ColorEngine,
}

impl AnimationEngine {
    pub fn new(ascii_text: String, duration_ms: u64, fps: u32) -> Self {
        Self {
            ascii_art: AsciiArt::new(ascii_text),
            duration_ms,
            fps,
            effect: Box::new(effects::FadeIn),
            easing: Box::new(easing::Linear),
            color_engine: ColorEngine::new(),
        }
    }

    pub fn with_effect(mut self, effect_name: &str) -> Result<Self> {
        self.effect = effects::get_effect(effect_name)?;
        Ok(self)
    }

    pub fn with_easing(mut self, easing_name: &str) -> Result<Self> {
        self.easing = easing::get_easing_function(easing_name)?;
        Ok(self)
    }

    pub fn with_color_engine(mut self, color_engine: ColorEngine) -> Self {
        self.color_engine = color_engine;
        self
    }

    pub async fn run(&self, terminal: &mut TerminalManager) -> Result<()> {
        let renderer = renderer::Renderer::new(
            &self.ascii_art,
            self.duration_ms,
            self.fps,
            &*self.effect,
            &*self.easing,
            &self.color_engine,
        );

        renderer.render(terminal).await
    }
}
