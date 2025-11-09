use anyhow::{bail, Result};

pub trait EasingFunction: Send + Sync {
    fn ease(&self, t: f64) -> f64;
    #[allow(dead_code)]
    fn name(&self) -> &str;
}

// Linear
pub struct Linear;
impl EasingFunction for Linear {
    fn ease(&self, t: f64) -> f64 {
        t
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "linear"
    }
}

// Basic easing
pub struct EaseIn;
impl EasingFunction for EaseIn {
    fn ease(&self, t: f64) -> f64 {
        t * t
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in"
    }
}

pub struct EaseOut;
impl EasingFunction for EaseOut {
    fn ease(&self, t: f64) -> f64 {
        t * (2.0 - t)
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out"
    }
}

pub struct EaseInOut;
impl EasingFunction for EaseInOut {
    fn ease(&self, t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out"
    }
}

// Quadratic
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
    fn ease(&self, t: f64) -> f64 {
        t * t
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-quad"
    }
}

pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
    fn ease(&self, t: f64) -> f64 {
        t * (2.0 - t)
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out-quad"
    }
}

pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
    fn ease(&self, t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out-quad"
    }
}

// Cubic
pub struct EaseInCubic;
impl EasingFunction for EaseInCubic {
    fn ease(&self, t: f64) -> f64 {
        t * t * t
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-cubic"
    }
}

pub struct EaseOutCubic;
impl EasingFunction for EaseOutCubic {
    fn ease(&self, t: f64) -> f64 {
        let t1 = t - 1.0;
        t1 * t1 * t1 + 1.0
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out-cubic"
    }
}

pub struct EaseInOutCubic;
impl EasingFunction for EaseInOutCubic {
    fn ease(&self, t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out-cubic"
    }
}

// Back
pub struct EaseInBack;
impl EasingFunction for EaseInBack {
    fn ease(&self, t: f64) -> f64 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        c3 * t * t * t - c1 * t * t
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-back"
    }
}

pub struct EaseOutBack;
impl EasingFunction for EaseOutBack {
    fn ease(&self, t: f64) -> f64 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out-back"
    }
}

pub struct EaseInOutBack;
impl EasingFunction for EaseInOutBack {
    fn ease(&self, t: f64) -> f64 {
        let c1 = 1.70158;
        let c2 = c1 * 1.525;
        if t < 0.5 {
            ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out-back"
    }
}

// Elastic
pub struct EaseInElastic;
impl EasingFunction for EaseInElastic {
    fn ease(&self, t: f64) -> f64 {
        if t == 0.0 {
            return 0.0;
        }
        if t == 1.0 {
            return 1.0;
        }
        let c4 = (2.0 * std::f64::consts::PI) / 3.0;
        -(2.0_f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-elastic"
    }
}

pub struct EaseOutElastic;
impl EasingFunction for EaseOutElastic {
    fn ease(&self, t: f64) -> f64 {
        if t == 0.0 {
            return 0.0;
        }
        if t == 1.0 {
            return 1.0;
        }
        let c4 = (2.0 * std::f64::consts::PI) / 3.0;
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out-elastic"
    }
}

pub struct EaseInOutElastic;
impl EasingFunction for EaseInOutElastic {
    fn ease(&self, t: f64) -> f64 {
        if t == 0.0 {
            return 0.0;
        }
        if t == 1.0 {
            return 1.0;
        }
        let c5 = (2.0 * std::f64::consts::PI) / 4.5;
        if t < 0.5 {
            -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
        } else {
            (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out-elastic"
    }
}

// Bounce
fn bounce_out(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

pub struct EaseInBounce;
impl EasingFunction for EaseInBounce {
    fn ease(&self, t: f64) -> f64 {
        1.0 - bounce_out(1.0 - t)
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-bounce"
    }
}

pub struct EaseOutBounce;
impl EasingFunction for EaseOutBounce {
    fn ease(&self, t: f64) -> f64 {
        bounce_out(t)
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-out-bounce"
    }
}

pub struct EaseInOutBounce;
impl EasingFunction for EaseInOutBounce {
    fn ease(&self, t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
        }
    }
    #[allow(dead_code)]
    fn name(&self) -> &str {
        "ease-in-out-bounce"
    }
}

pub fn get_easing_function(name: &str) -> Result<Box<dyn EasingFunction>> {
    match name {
        "linear" => Ok(Box::new(Linear)),
        "ease-in" => Ok(Box::new(EaseIn)),
        "ease-out" => Ok(Box::new(EaseOut)),
        "ease-in-out" => Ok(Box::new(EaseInOut)),
        "ease-in-quad" => Ok(Box::new(EaseInQuad)),
        "ease-out-quad" => Ok(Box::new(EaseOutQuad)),
        "ease-in-out-quad" => Ok(Box::new(EaseInOutQuad)),
        "ease-in-cubic" => Ok(Box::new(EaseInCubic)),
        "ease-out-cubic" => Ok(Box::new(EaseOutCubic)),
        "ease-in-out-cubic" => Ok(Box::new(EaseInOutCubic)),
        "ease-in-back" => Ok(Box::new(EaseInBack)),
        "ease-out-back" => Ok(Box::new(EaseOutBack)),
        "ease-in-out-back" => Ok(Box::new(EaseInOutBack)),
        "ease-in-elastic" => Ok(Box::new(EaseInElastic)),
        "ease-out-elastic" => Ok(Box::new(EaseOutElastic)),
        "ease-in-out-elastic" => Ok(Box::new(EaseInOutElastic)),
        "ease-in-bounce" => Ok(Box::new(EaseInBounce)),
        "ease-out-bounce" => Ok(Box::new(EaseOutBounce)),
        "ease-in-out-bounce" => Ok(Box::new(EaseInOutBounce)),
        _ => bail!("Unknown easing function: {}", name),
    }
}
