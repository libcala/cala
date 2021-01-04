// Cala
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! User Input.
//!
//! # Getting Started
//! ```rust,no_run
//! use cala::exec::{exec, wait};
//! use cala::log::{log, Tag};
//! use cala::input::Input;
//!
//! const INFO: Tag = Tag::new("Info").show(true);
//!
//! /// The program's shared state.
//! struct State {}
//!
//! /// Event handled by the event loop.
//! enum Event {
//!     Input(Input),
//! }
//!
//! impl State {
//!     /// Event loop.
//!     fn event(&mut self, event: Event) {
//!         match event {
//!             Event::Input(input) => log!(INFO, "Input: {:?}", input),
//!         }
//!     }
//! }
//!
//! /// Start the async executor.
//! fn main() {
//!     let mut state = State {};
//!     let mut input = human::Input::listener();
//!
//!     exec!(state.event(wait! {
//!         Event::Input((&mut input).await),
//!     }));
//! }
//! ```

pub use human::*;
