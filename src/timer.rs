/// A timer for timing animations, etc.
#[derive(Copy, Clone)]
pub struct TimedLoop {
    // Incremented every `add()`.
    counter: f64,
    // `counter` wraps around
    maximum: f64,
}

impl TimedLoop {
    /// Create a repeating timer over `secs` seconds.
    #[allow(clippy::cast_lossless)] // const fn doesn't support u64::from() yet
    pub const fn new(secs: f64) -> TimedLoop {
        TimedLoop {
            counter: 0.0,
            maximum: secs,
        }
    }

    /// Add time to the `TimedLoop`.  Returns float for use in animations.
    pub fn add(&mut self, dt: f64) -> f32 {
        self.counter = (self.counter + dt) % 1.0;
        (*self).into()
    }
}

impl Into<f32> for TimedLoop {
    fn into(self) -> f32 {
        (self.counter / self.maximum) as f32
    }
}
