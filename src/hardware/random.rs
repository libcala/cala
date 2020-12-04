//! **feature:random** - Generate random numbers.
//!
//! # Usage
//! ```rust
//! use cala::{
//!     log::{log, Tag},
//!     random::{Rng, WyRand},
//! };
//! 
//! const INFO: Tag = Tag::new("Info");
//!
//! let mut rng = WyRand::new();
//! log!(INFO, "Random number between 1 and 100: {}", rng.generate_range::<u64>(1, 100));
//! ```

pub use nanorand::{RNG as Rng, WyRand, Pcg64, ChaCha};
