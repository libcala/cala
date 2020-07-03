//! **feature:microphone** - Audio capture (recording) device
//!
//! # Usage
//! The following example shows how to play back audio as it's being recorded.
//! Headphones recommended in order to avoid feedback.
//!
//! ```rust
//! use std::collections::VecDeque;
//!
//! // The program data context.
//! struct Data {
//!     buffer: VecDeque<(i16, i16)>,
//! }
//!
//! // Set the home loop to `run()`.
//! cala::init!(run, Data {
//!     buffer: VecDeque::new(),
//! });
//!
//! fn run(data: &mut Data) -> cala::Loop<Data> {
//!     // Record some sound.
//!     cala::record(&mut |_whichmic, l, r| {
//!         data.buffer.push_back((l, r));
//!     });
//!
//!     // Play that sound.
//!     cala::play(&mut || {
//!         if let Some((lsample, rsample)) = data.buffer.pop_front() {
//!             cala::AudioSample::stereo(lsample, rsample)
//!         } else {
//!             // Play silence if not enough has been recorded yet.
//!             cala::AudioSample::stereo(0, 0)
//!         }
//!     });
//!
//!     cala::Continue
//! }
//! ```

pub use wavy::{Recorder, S16LEx2};
