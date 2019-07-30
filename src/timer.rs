/// A timer for timing animations, etc.
#[derive(Copy, Clone)]
pub struct TimedLoop {
    // Incremented every `add()`.
    counter: u64,
    // `counter` wraps around
    maximum: u64,
}

impl TimedLoop {
    /// Seconds and nanoseconds (1 / 1_000_000_000 of a second).
    pub const fn new(secs: u32, nanos: u32) -> TimedLoop {
        let whol = secs as u64 * 1_000_000_000u64;
        let frac = nanos as u64;
        let value = whol + frac;

        TimedLoop {
            counter: 0,
            maximum: value,
        }
    }

    /// Add time to the `TimedLoop`.  Returns float for use in animations.
    pub fn add(&mut self) -> f32 {
        let mut nanos = crate::delta();
        let left = self.maximum - self.counter;
        if nanos > left {
            nanos -= left + 1;
            self.counter = nanos;
        } else {
            self.counter += nanos;
        }
        (*self).into()
    }
}

impl Into<f32> for TimedLoop {
    fn into(self) -> f32 {
        (self.counter as f64 / self.maximum as f64) as f32
    }
}
