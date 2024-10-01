use std::time::{Duration, Instant};
pub struct FpsCounter {
    last_second_frames: Vec<Instant>,
    last_frame_time: Instant,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_second_frames: Vec::new(),
            last_frame_time: Instant::now(),
        }
    }
    pub fn update(&mut self) -> (f64, u32) {
        let now = Instant::now();
        let frame_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.last_second_frames.push(now);
        self.last_second_frames
            .retain(|&t| now - t < Duration::from_secs(1));
        let fps = self.last_second_frames.len() as f64;
        let frame_time_ms = frame_time.as_secs_f64() * 1000.0;
        (frame_time_ms, fps as u32)
    }
}
