// Cala
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Video types.
//!
//! # Getting Started
//! The following code generates an HWB gradient.  If you want to save it to a
//! file, consider using [png_pong](https://crates.io/crates/png_pong).
//!
//! ```rust
//! use pix::hwb::SHwb8;
//! use pix::rgb::SRgb8;
//! use pix::Raster;
//! 
//! let mut r = Raster::with_clear(256, 256);
//! for (y, row) in r.rows_mut(()).enumerate() {
//!     for (x, p) in row.iter_mut().enumerate() {
//!         let h = ((x + y) >> 1) as u8;
//!         let w = y.saturating_sub(x) as u8;
//!         let b = x.saturating_sub(y) as u8;
//!         *p = SHwb8::new(h, w, b);
//!     }
//! }
//! // Convert to SRgb8 pixel format
//! let raster = Raster::<SRgb8>::with_raster(&r);
//! ```

pub use pix::*;
