//! ![Siyo](https://free.plopgrizzly.com/siyo/banner.svg)
//!
//! # About
//! Cross-platform system interface for hardware IO. Use screens, speakers, microphones, input devices, etc.
//!
//! ## Motivation
//! There needs to be crate that can handle all of the common tasks that are currently not portable across different platforms.
//!
//! ## Naming
//! Siyo is a cross-platform system interface for hardware IO.  If you try to say that really fast and *accidently* miss a couple syllables/words it might sound like siyo (S+AH+YO).
//!
//! ## Features
//! Each interface is represented by a module in Siyo.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://free.plopgrizzly.com/siyo/icon.svg",
    html_favicon_url = "https://free.plopgrizzly.com/siyo/icon.svg"
)]

mod shared;

/// Fixed point, SIMD, GPU, Math
pub mod math;

/// Date, Time of day, Timer
pub mod clock;

/// Human Interface Device, USB
pub mod hid;

/// Make a window
pub mod screen;

/// Camera, Webcam
pub mod cam;

/// Play sound through speakers
pub mod speaker;

/// Record sound through microphone
pub mod mic;

/// Network
pub mod net;

/// Persistent Storage
pub mod drive;

pub use crate::shared::{SIYO_HIDDEN_GLOBAL__, SiyoHiddenGlobal__, siyo_hidden__};
