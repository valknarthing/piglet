use anyhow::{Result, bail};
use scirs2_interpolate::*;

pub trait EasingFunction: Send + Sync {
    fn ease(&self, t: f64) -> f64;
    fn name(&self) -> &str;
}

// Linear
pub struct Linear;
impl EasingFunction for Linear {
    fn ease(&self, t: f64) -> f64 { t }
    fn name(&self) -> &str { "linear" }
}

// Quadratic
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
    fn ease(&self, t: f64) -> f64 { quad_ease_in(t, 0.0, 1.0, 1.0) }
    fn name(&self) -> &str { "ease-in-quad" }
}

pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
    fn ease(&self, t: f64) -> f64 { quad_ease_out(t, 0.0, 1.0, 1.0) }
    fn name(&self) -> &str { "ease-out-quad" }
}

pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
    fn ease(&self, t: f64) -> f64 { quad_ease_in_out(t, 0.0, 1.0, 1.0) }
    fn name(&self) -> &str { "ease-in-out-quad" }
}

// Cubic
pub struct EaseInCubic;
impl EasingFunction for EaseInCubic {
    fn ease(&self, t: f64) -> f64 { cubic_ease_in(t, 0.0, 1.0, 1.0) }
    fn name(&self) -> &str { "ease-in-cubic" }
}

pub struct EaseOutCubic;
impl EasingFunction for EaseOutCubic {
    fn ease(&self, t: f64) -> f64 { cubic_ease_out(t, 0.0, 1.0, 1.0) }
    fn name(&self) -> &str { "ease-out-cubic" }
}

pub struct EaseInOutCubic;
impl EasingFunction for EaseInOut