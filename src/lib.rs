//! <p align="center">
//!   <img alt="Cala" src="https://libcala.github.io/logo.svg">
//! </p>
//! <p align="center">
//! <a href="https://docs.rs/cala"><img src="https://docs.rs/cala/badge.svg"></a>
//! <a href="https://travis-ci.com/libcala/cala"><img src="https://api.travis-ci.com/libcala/cala.svg?branch=master" alt="Cala Build Status"></a>
//! <a href="https://crates.io/crates/cala"><img src="https://img.shields.io/crates/v/cala.svg" alt = "cala on crates.io"></a>
//! <a href="https://discord.gg/nXwF59K"><img src="https://img.shields.io/badge/discord-join%20server-green.svg" alt="Discord"></a>
//! <br>
//!   <strong><a href="https://libcala.github.io">Website</a> | <a href="https://github.com/libcala/cala">GitHub</a> | <a href="https://libcala.github.io/changelog">Changelog</a> | <a href="https://libcala.github.io/tutorials">Tutorials</a> </strong>
//! </p>
//!
//! # Getting Started
//! Each hardware interface can be enabled with a feature.  For example, If you
//! want to use the `draw` module and the `screen` module, you might put this in
//! your `Cargo.toml`:
//!
//! <p style="width:100%"><pre style="width:100%"><code style="width:100%"><span style="font-weight:bold;">[dependencies.cala]</span>
//! <span style="color:#0A0;font-weight:bold;">version</span> = <span style="color:#0A0">"0.8"</span>
//! <span style="color:#0A0;font-weight:bold;">features</span> = [<span style="color:#0A0">"draw"</span>, <span style="color:#0A0">"screen"</span>]</code></pre></p>
//!
//! Here's the boilerplate for your main.rs (you probably want to put modules
//! in separate files):
//!
//! ```rust,no_run
//! // When no features are enabled, only imports prelude traits.
//! use cala::*;
//!
//! exec!(async_main); // Set entry point for the app.
//!
//! enum State {
//!     A(a::State), // The only state
//! }
//! 
//! async fn event_loop(state: &mut State) {
//!     use State::*;
//!     match state {
//!         A(state) => state.event_loop().await,
//!     }
//! }
//! 
//! // Entry point
//! async fn async_main() {
//!     let mut state = State { };
//!     loop { event_loop(&mut state).await }
//! }
//!
//! // This module contains state A's structure and event loop.
//! mod a {
//!     use cala::*;
//!
//!     // Data associated with state A.
//!     struct State { }
//!
//!     impl State {
//!         // State A's event loop
//!         async fn event_loop(&mut self) {
//!             // Leaving this empty will result in the async
//!             // executor going to sleep.
//!             [/*put futures here*/].select().await
//!         }
//!     }
//! }
//! ```
//! 
//! Module documentation may include simple tutorials.  More in depth tutorials
//! may be found <a href="https://libcala.github.io/tutorials">here</a>.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

/////////////////
//// Modules ////
/////////////////

// Private
mod exec;
mod hardware;
mod prelude;

pub use prelude::*;
pub use hardware::*;

// Hidden, because only used in macros.
#[doc(hidden)]
pub mod __hidden {
    #[cfg(feature = "pasts")]
    pub use pasts::{CvarExec, Executor};
    #[cfg(feature = "draw")]
    pub use crate::hardware::draw::__hidden::draw_thread;
}

// mod icons; // FIXME Do somethign with the GUI icons

/////////////////////
//// End of File ////
/////////////////////
