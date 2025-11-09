use std::time::{Duration, Instant};

pub struct Timeline {
    duration_ms: u64,
    fps: u32,
    start_time: Option<Instant>,
    current_frame: usize,
    total_frames: usize,
}

impl Timeline {
    pub fn new(duration_ms: u64, fps: u32) -> Self {
        let total_frames = ((duration_ms as f64 / 1000.0) * fps as f64).ceil() as usize;

        Self {
            duration_ms,
            fps,
            start_time: None,
            current_frame: 0,
            total_frames,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.current_frame = 0;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.start_time = None;
        self.current_frame = 0;
    }

    pub fn is_complete(&self) -> bool {
        self.current_frame >= self.total_frames
    }

    pub fn progress(&self) -> f64 {
        if self.total_frames == 0 {
            return 1.0;
        }
        (self.current_frame as f64 / self.total_frames as f64).min(1.0)
    }

    pub fn next_frame(&mut self) -> bool {
        if self.is_complete() {
            return false;
        }

        self.current_frame += 1;
        true
    }

    pub fn frame_duration(&self) -> Duration {
        Duration::from_millis(1000 / self.fps as u64)
    }

    #[allow(dead_code)]
    pub fn elapsed(&self) -> Duration {
        self.start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    #[allow(dead_code)]
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    #[allow(dead_code)]
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new(1000, 30);
        assert_eq!(timeline.total_frames(), 30);
        assert_eq!(timeline.fps(), 30);
    }

    #[test]
    fn test_timeline_progress() {
        let mut timeline = Timeline::new(1000, 10);
        timeline.start();

        assert_eq!(timeline.progress(), 0.0);

        for _ in 0..5 {
            timeline.next_frame();
        }

        assert_eq!(timeline.progress(), 0.5);
    }

    #[test]
    fn test_timeline_completion() {
        let mut timeline = Timeline::new(1000, 10);
        timeline.start();

        assert!(!timeline.is_complete());

        for _ in 0..10 {
            timeline.next_frame();
        }

        assert!(timeline.is_complete());
    }
}
