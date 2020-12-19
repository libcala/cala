//! **feature:log** - Text output through some medium (stdout, web console,
//! serial, etc.)
//!
//! # Usage
//! ```rust
//! use devout::{log, Tag};
//! 
//! const INFO: Tag = Tag::new("Info").show(true);
//! 
//! log!(INFO, "Result: {}", 4.4);
//! ```

pub use devout::{log, Tag};
