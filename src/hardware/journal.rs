//! **feature:journal** - Text output through some medium (stdout, web console,
//! serial, etc.)
//!
//! # Usage
//! ```rust
//! use devout::{dev, out};
//!
//! const INFO: &str = "Info";
//!
//! // Prints twice in development, once in production.
//! dev!(INFO, "Result: {}", 4.4);
//! out!(INFO, "Result: {}", 4.4);
//! ```

pub use devout::{dev, out};
