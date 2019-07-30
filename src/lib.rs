//! <p align="center">
//! <img src="https://jeronaldaron.github.io/cala/icon.svg" alt="Cala" width="256px" height="256px">
//! </p>
//!
//! ### Note
//! Cala is a complete redesign of previous library [ADI](https://crates.io/crates/adi).  It is still in it's early stages.
//!
//! # About
//! Easily create cross-platform applications.  Some common tasks are not easily portable across different platforms, and this crate hopes to fix that.  That way you don't have to worry about how to port your GUI, audio, or bluetooth interface, etc. and can dive right in to building your application's content!
//!
//! Dive is a platform-agnostic system interface for hardware IO.  This means that eventually, Dive should support all of the different hardware that's connected to your computer.  Dive is designed so that it talks to the operating system to interface with the hardware, so no special permissions are needed for your application.
//!
//! ## Features
//! * Targeted Platforms: Linux (includes Raspberry Pi), MacOS, Redox, Android, Windows, iOS, Web (WASM), Nintendo Switch, XBox, PlayStation, FreeBSD, others (Maybe FreeDOS for fun 😉️).
//! * Getting user information (Linux, Windows, MacOS)
//! * Playing / recording audio (Linux)
//! * Filesystem loading / saving ZIP files (Linux, Windows)
//! * Game Controller - JoyStick (Linux)
//! * Hardware acceleration - SIMD, GPU (NOT IMPLEMENTED YET)
//! * Clock - Date, Time of day, Timer (NOT IMPLEMENTED YET)
//! * GUI - Render, Mouse & Keyboard (NOT IMPLEMENTED YET)
//! * Camera - Webcam (NOT IMPLEMENTED YET)
//! * Network - Bluetooth & Wifi Direct (NOT IMPLEMENTED YET)
//!
//! # Getting Started
//! * TODO
//!
//! ## Features
//! Each hardware interface can be enabled with a feature.  By default, all features are
//! enabled.  There is a module for each feature (feature and module names match).

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://jeronaldaron.github.io/cala/icon.svg",
    html_favicon_url = "https://jeronaldaron.github.io/cala/icon.svg"
)]

mod run;

#[cfg(feature = "user")]
pub mod user {
    //! API for getting user information.  Enable with the `user` feature.
    //!
    //! # Usage
    //! ```rust
    //! // Set the home loop to `run()`.
    //! cala::loop_init!(run, ());
    //!
    //! // Function that runs while your app runs.
    //! pub fn run(_: &mut ()) -> cala::Loop<()> {
    //!     // Print out the user's information.
    //!     println!("{}", cala::user());
    //!     // Exit.
    //!     cala::Exit
    //! }
    //! ```

    include!("internal/whoami.rs");
}

#[cfg(feature = "controller")]
pub mod controller {
    //! API for getting joystick / controller / gamepad input.  Enable with the
    //! `controller` feature.
    //!
    //! # Usage
    //! ```rust
    //! // Set the home loop to `run()`.
    //! cala::loop_init!(run, ());
    //!
    //! // Function that runs while your app runs.
    //! pub fn run(_: &mut ()) -> cala::Loop<()> {
    //!     let layout = cala::ControllerLayout::new().joy(false).lrt(false).abxy(false);
    //!
    //!     // Iterate through all of the controllers.
    //!     'a: for (id, state) in cala::controllers(&layout) {
    //!         println!("{}: {:?}", id, state.get());
    //!     }
    //!     std::thread::sleep(std::time::Duration::from_millis(16));
    //!     // Exit.
    //!     cala::Continue
    //! }
    //! ```

    include!("internal/stick.rs");
}

#[cfg(feature = "audio")]
pub mod audio {
    //! API for recording / playing audio.  Enable with the `audio` feature.
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
    //! cala::loop_init!(run, Data {
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

#[cfg(feature = "files")]
pub mod files {
    //! API for loading & saving files.  Enable with the `files` feature.
    //!
    //! # Usage
    //! ```rust
    //! // TODO
    //! ```

    include!("internal/stronghold.rs");
}

// Export all types to root.
pub use run::Loop;

#[cfg(feature = "user")]
#[doc(hidden)]
pub use user::*;

#[cfg(feature = "controller")]
#[doc(hidden)]
pub use controller::*;

#[cfg(feature = "audio")]
#[doc(hidden)]
pub use audio::*;

#[doc(hidden)]
pub use internal::init;
#[doc(hidden)]
pub use run::Loop::*;

//mod audio;
// mod dive;
mod internal;
// mod iolock;

// Others....
//pub use audio::AudioSample;

/// Define the entry point for your program.
///
/// Note that not only is the function an entry point, but also a loop.  Usually you'll
/// want to do your initialization in a block of code for the second parameter.  You can
/// also do additional intialization inside of the initial loop, and then at the end of
/// the function switch to your main loop.
///
/// See [`Loop`](enum.Loop.html) for more details.
///
/// # Usage
/// ```rust
/// // Set the home loop to `run()`.
/// cala::loop_init!(run, ());
///
/// // Function that runs while your app runs.
/// pub fn run(_: &mut ()) -> cala::Loop<()> {
///     // Print out the user's information.
///     println!("{}", cala::user());
///     // Exit.
///     cala::Exit
/// }
/// ```
#[macro_export]
macro_rules! loop_init {
    ($home_loop: expr, $init_data: expr) => {
        fn main() {
            cala::init();

            let mut current_loops: Vec<fn(&mut _) -> cala::Loop<_>> = vec![$home_loop];

            let mut data = { $init_data };

            'a: loop {
                match current_loops[current_loops.len() - 1](&mut data) {
                    cala::Exit => {
                        break 'a;
                    }
                    cala::Continue => { /* do nothing */ }
                    cala::Back => {
                        if current_loops.pop().is_none() {
                            break 'a;
                        }
                    }
                    cala::Replace(loop_a) => {
                        if current_loops.pop().is_none() {
                            break 'a;
                        }
                        current_loops.push(loop_a);
                    }
                    cala::Append(loop_a) => {
                        current_loops.push(loop_a);
                    }
                    cala::ReplaceWithBack(loop_a, loop_b) => {
                        if current_loops.pop().is_none() {
                            break 'a;
                        }
                        current_loops.push(loop_b);
                        current_loops.push(loop_a);
                    }
                }
            }
        }
    };
}
