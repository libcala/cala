// Cala
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Audio types.
//!
//! # Getting Started
//! The following code generates an 8-bit sawtooth wave.  For more advanced
//! audio synthesis, consider using [twang](https://crates.io/crates/twang).
//!
//! ```rust
//! use cala::audio::chan::Ch8;
//! use cala::audio::mono::Mono8;
//! use cala::audio::stereo::Stereo16;
//! use cala::audio::{Audio, Frame};
//!
//! let mut a = Audio::<Mono8>::with_silence(44_100, 256);
//! for (i, s) in a.iter_mut().enumerate() {
//!     s.channels_mut()[0] = Ch8::new(i as i8);
//! }
//! // Convert to stereo 16-Bit 48_000 KHz audio format
//! let audio = Audio::<Stereo16>::with_stream(48_000, &a);
//! ```

pub use fon::*;
