// Cala
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
//! Each module needs to be enabled with a feature.  For example, if you want to
//! use the `task` module, put this in your *Cargo.toml*:
//!
//! <p style="width:100%"><pre style="width:100%"><code style="width:100%"><span style="font-weight:bold;">[dependencies.cala]</span>
//! <span style="color:#0A0;font-weight:bold;">version</span> = <span style="color:#0A0">"0.8"</span>
//! <span style="color:#0A0;font-weight:bold;">features</span> = [<span style="color:#0A0">"task"</span>]</code></pre></p>
//!
//! Here's the boilerplate for your main.rs:
//!
//! ```rust,no_run
//! use cala::task::{exec, wait, never};
//!
//! /// The program's shared state.
//! struct State {}
//!
//! /// Event handled by the event loop.
//! enum Event {
//!     Never(()),
//! }
//!
//! impl State {
//!     /// Event loop.
//!     fn event(&mut self, event: Event) {
//!         match event {
//!             Event::Never(_) => unreachable!(),
//!         }
//!     }
//! }
//!
//! /// Start the async executor.
//! fn main() {
//!     let mut state = State {};
//!     let mut never = never();
//!
//!     exec!(state.event(wait! {
//!         Event::Never((&mut never).await),
//!     }));
//! }
//! ```
//!
//! Module documentation may include simple tutorials.  More in depth tutorials
//! may be found <a href="https://libcala.github.io/tutorials">here</a>.
//!
//! # A Tour of Cala
//! The rest of this crate documentation is dedicated to pointing out notable
//! features of the Cala crate.
//!
//! ## Containers and Collections
//! The [`audio`](crate::audio) and [`video`](crate::video) modules contain
//! multimedia types for working with sounds and graphics.
//!
//! ## Platform Abstractions
//! The [`task`](crate::task) module contains abstractions for dealing with
//! asynchronous code.
//!
//! The [`gui`](crate::gui) module contains abstractions for making a GUI
//! (Graphical User Interface).
//!
//! ## I/O
//! Cala's main purpose is to abstract over differences in common platforms,
//! notably Windows, Web, and Unix derivatives (including mobile) for things
//! that the standard library does not.  This is mostly with multi-media I/O,
//! defined in these modules:
//!  - [`bluetooth`](crate::bluetooth) - bluetooth
//!  - [`camera`](crate::camera) - webcam, phone camera
//!  - [`client`](crate::client) - client network communication
//!  - [`database`](crate::database) - database for persistent storage
//!  - [`graphics`](crate::graphics) - hardware-accelerated graphics rendering
//!  - [`haptic`](crate::haptic) - haptic force feedback
//!  - [`info`](crate::info) - system environment information
//!  - [`input`](crate::input) - user input.
//!  - [`log`](crate::log) - message logging
//!  - [`microphone`](crate::microphone) - microphone input
//!  - [`port`](crate::port) - general purpose I/O ports
//!  - [`random`](crate::random) - random number generators
//!  - [`server`](crate::server) - server network communication
//!  - [`speakers`](crate::speakers) - speaker output
//!  - [`timer`](crate::timer) - using timers
//!  - [`usb`](crate::usb) - universal serial bus communcations
//!  - [`when`](crate::when) - getting the current time
//!  - [`window`](crate::window) - display graphics in an area on a screen.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

// Private
#[cfg(any(feature = "client", feature = "server"))]
mod net;

// Public
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "bluetooth")]
pub mod bluetooth;
#[cfg(feature = "camera")]
pub mod camera;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "graphics")]
pub mod graphics;
#[cfg(feature = "gui")]
pub mod gui;
#[cfg(feature = "haptic")]
pub mod haptic;
#[cfg(feature = "info")]
pub mod info;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "microphone")]
pub mod microphone;
#[cfg(feature = "port")]
pub mod port;
#[cfg(feature = "random")]
pub mod random;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "speakers")]
pub mod speakers;
#[cfg(feature = "task")]
pub mod task;
#[cfg(feature = "timer")]
pub mod timer;
#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "video")]
pub mod video;
#[cfg(feature = "when")]
pub mod when;
#[cfg(feature = "window")]
pub mod window;
