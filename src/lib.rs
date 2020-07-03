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
//! want to use the `audio` module and the `time` module, you might put this in
//! your `Cargo.toml`:
//!
//! <p style="width:100%"><pre style="width:100%"><code style="width:100%"><span style="font-weight:bold;">[dependencies.cala]</span>
//! <span style="color:#0A0;font-weight:bold;">version</span> = <span style="color:#0A0">"0.8"</span>
//! <span style="color:#0A0;font-weight:bold;">features</span> = [<span style="color:#0A0">"audio"</span>, <span style="color:#0A0">"time"</span>]</code></pre></p>
//!
//! Module documentation may include simple tutorials.  More in depth tutorials
//! may be found <a href="https://libcala.github.io/tutorials">here</a>.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

#[doc(hidden)]
pub mod __hidden {
    #[cfg(feature = "pasts")]
    pub use pasts::{CvarExec, Executor};

    #[cfg(feature = "draw")]
    pub use crate::hardware::draw::__hidden::draw_thread;
}

pub mod prelude {
    //! Automatically import traits with `use cala::prelude::*;`.

    #[cfg(feature = "draw")]
    pub use pix::{ops::Blend as PixelBlend, el::Pixel};
    
    #[cfg(feature = "draw")]
    /// Something that can be drawn on.
    pub trait Canvas {
        /// Draw a group on the screen.
        fn draw(&mut self, shader: &crate::draw::Shader, group: &std::sync::Arc<crate::draw::Group>);
        /// Set camera for shader.
        fn set_camera(&mut self, shader: &crate::draw::Shader, camera: crate::draw::Transform);
        /// Set tint for shader.
        fn set_tint<P: pix::el::Pixel>(&mut self, shader: &crate::draw::Shader, tint: P)
        where
            pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>;
        /// Draw a group with a texture on the screen.
        fn draw_graphic(
            &mut self,
            shader: &crate::draw::Shader,
            group: &std::sync::Arc<crate::draw::Group>,
            graphic: &crate::draw::Texture,
        );
        /// Returns the amount of time elapsed since the previous frame. 
        fn elapsed(&self) -> std::time::Duration;
        /// Return the aspect ratio of the `Canvas`.
        fn aspect(&self) -> f32;
    }

    #[cfg(feature = "pasts")]
    pub use pasts::{DynFut as IntoDynFut, Join as JoinFut, Select as SelectFut};

    #[cfg(feature = "pasts")]
    /// Trait for spawning tasks in a thread pool to run closures as a `Future`.
    pub trait SpawnBlocking<T> {
        /// Turn closure into a future.
        fn spawn_blocking(self) -> Box<dyn std::future::Future<Output = T>>;
    }

    impl<T, F> SpawnBlocking<T> for F
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        fn spawn_blocking(self) -> Box<dyn std::future::Future<Output = T>> {
            Box::new(pasts::spawn_blocking(self))
        }
    }
}

mod exec;
mod hardware;

pub use hardware::*;

/* **** */

#[cfg(feature = "timer")]
mod timer;

#[cfg(feature = "user")]
pub mod user {
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
}

#[cfg(feature = "audio")]
pub mod audio {
    //! **feature:audio** - Record and/or play audio.
    //!
    //! # Usage
    //! The following example shows how to play audio as it's being recorded.  Headphones
    //! recommended.
    //!
    //! ```rust
    //! use std::collections::VecDeque;
    //!
    //! // The program data context.
    //! struct Data {
    //!     buffer: VecDeque<(i16, i16)>,
    //! }
    //!
    //! // Set the home loop to `run()`.
    //! cala::init!(run, Data {
    //!     buffer: VecDeque::new(),
    //! });
    //!
    //! fn run(data: &mut Data) -> cala::Loop<Data> {
    //!     // Record some sound.
    //!     cala::record(&mut |_whichmic, l, r| {
    //!         data.buffer.push_back((l, r));
    //!     });
    //!
    //!     // Play that sound.
    //!     cala::play(&mut || {
    //!         if let Some((lsample, rsample)) = data.buffer.pop_front() {
    //!             cala::AudioSample::stereo(lsample, rsample)
    //!         } else {
    //!             // Play silence if not enough has been recorded yet.
    //!             cala::AudioSample::stereo(0, 0)
    //!         }
    //!     });
    //!
    //!     cala::Continue
    //! }
    //! ```

    include!("internal/wavy.rs");
}

#[cfg(feature = "journal")]
pub mod journal {
    //! **feature:journal** - Text output through some medium (stdout, web
    //! console, serial, etc.)
    //!
    //! # Usage
    //! ```rust
    //! // TODO
    //! ```

    pub use devout::{dev, out};
}

#[cfg(feature = "files")]
pub mod files {
    //! **feature:files** - Load & save files.
    //!
    //! # Usage
    //! ```rust
    //! // TODO
    //! ```

    include!("internal/stronghold.rs");
}

#[cfg(feature = "draw")]
mod icons;

#[cfg(feature = "time")]
pub mod time;

pub use exec::*;

#[cfg(feature = "user")]
#[doc(hidden)]
pub use user::*;

#[cfg(feature = "input")]
#[doc(hidden)]
pub use input::*;

#[cfg(feature = "journal")]
#[doc(hidden)]
pub use journal::*;

#[cfg(feature = "audio")]
#[doc(hidden)]
pub use audio::*;

#[cfg(feature = "draw")]
#[doc(hidden)]
pub use draw::*;

#[cfg(feature = "time")]
#[doc(hidden)]
pub use time::*;

// mod dive;
