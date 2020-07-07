//! **feature:user** - Retrieve user information.
//!
//! # Usage
//! ```rust
//! use cala::*;
//!
//! // Set function that runs while your app runs.
//! exec!(run);
//! async fn run() {
//!     // Get the user's username.
//!     println!("{}", user::username());
//! }
//! ```

pub use whoami::{
    desktop_env, devicename, distro, hostname, platform, realname,
    username, DesktopEnv, Platform,
};
