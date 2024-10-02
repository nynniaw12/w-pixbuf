use chrono::{DateTime, Duration, Utc};

pub struct FpsCounter {
    last_second_frames: Vec<DateTime<Utc>>,
    last_frame_time: DateTime<Utc>,
    frame_time_ms: f64,
    fps: f64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_second_frames: Vec::new(),
            last_frame_time: Utc::now(),
            frame_time_ms: 0.0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) -> (f64, f64) {
        let now = Utc::now();
        let frame_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.last_second_frames.push(now);
        self.last_second_frames
            .retain(|&t| now - t < Duration::seconds(1));
        self.fps = self.last_second_frames.len() as f64;
        self.frame_time_ms = frame_time.num_milliseconds() as f64;
        (self.frame_time_ms, self.fps)
    }

    pub fn fps(&self)->f64{
        self.fps
    }

    pub fn frame_time(&self)->f64{
        self.frame_time_ms
    }
}
