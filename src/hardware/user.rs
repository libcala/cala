//! **feature:user** - Retrieve user information.
//!
//! # Usage
//! ```rust
//! use cala::*;
//!
//! fn main() {
//!     // Get the user's username.
//!     println!("{}", user::username());
//! }
//! ```

pub use whoami::{
    desktop_env, devicename, distro, hostname, platform, realname, username,
    DesktopEnv, Platform,
};
