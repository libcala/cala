// Cala
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Execution of asynchronous tasks.
//!
//! # Getting Started
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

use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;

pub use pasts::{exec, join, wait, race};

struct Never;

impl Future for Never {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}

/// An asynchronous task that never finishes.
pub fn never() -> impl Future<Output = ()> + Unpin {
    Never
}
