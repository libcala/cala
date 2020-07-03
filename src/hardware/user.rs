//! **feature:user** - Retrieve user information.
//!
//! # Usage
//! ```rust
//! // Set the home loop to `run()`.
//! cala::init!(run, ());
//!
//! // Function that runs while your app runs.
//! pub fn run(_: &mut ()) -> cala::Loop<()> {
//!     // Get the user's username.
//!     println!("{}", cala::username());
//!     // Exit.
//!     cala::Exit
//! }
//! ```

pub use whoami::{
    desktop_env, devicename, distro, hostname, platform, realname,
    username, DesktopEnv, Platform,
};
