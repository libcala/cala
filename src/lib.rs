//! <p align="center">
//! <img src="https://aldarobot.plopgrizzly.com/cala/icon.svg" alt="Cala" width="256px" height="256px">
//! </p>
//!
//! ### Note
//! Cala is a complete redesign of previous library [ADI](https://crates.io/crates/adi).  It is still in it's early stages.
//!
//! # About
//! Easily create cross-platform applications.  Some common tasks are not easily portable across different platforms, and this crate hopes to fix that.  That way you don't have to worry about how to port your GUI, audio, or bluetooth interface, etc.  and can get straight to building your application's content!
//!
//! Cala is a platform-agnostic system interface for hardware IO.  This means that eventually, Cala should support all of the different hardware that's connected to your computer.  Cala is designed so that it talks to the operating system to interface with the hardware, so no special permissions are needed for your application.
//!
//! ## Features
//! - Targeted Platforms: Linux (includes Raspberry Pi), MacOS, Redox, Android, Windows, iOS, Web (WASM), Nintendo Switch, XBox, PlayStation, FreeBSD, others (Maybe FreeDOS for fun 😉️).
//! - Getting user information (Linux, Windows, MacOS)
//! - Playing / recording audio (Linux)
//! - Filesystem loading / saving ZIP files (Linux, Windows)
//! - Game Controller - JoyStick (Linux)
//! - Clock - Date, Time of day, Timer (All Platforms)
//! - Graphics - Render and User Interface (Linux)
//! - Camera - Webcam (NOT IMPLEMENTED YET)
//! - Hardware acceleration - SIMD, GPU (NOT IMPLEMENTED YET)
//! - Network - Bluetooth & Wifi Direct (NOT IMPLEMENTED YET)
//!
//! # Getting Started
//! * TODO
//!
//! ## Features
//! Each hardware interface can be enabled with a feature.  By default, all features are
//! enabled.  There is a module for each feature (feature and module names match).

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://aldarobot.plopgrizzly.com/cala/icon.svg",
    html_favicon_url = "https://aldarobot.plopgrizzly.com/cala/icon.svg"
)]

mod run;

#[cfg(feature = "timer")]
mod timer;

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

#[cfg(feature = "graphics")]
#[macro_use]
pub mod graphics {
    //! API for rendering graphics.  Enable with the `graphics` feature.
    //!
    //! # Getting Started
    //! This API is designed to be high-level without sacrificing optimization.
    //! Graphics are complicated though, so before you start, a few things need
    //! to be defined.
    //!
    //! ## Shader
    //! A Shader is a program that runs on the GPU for the purpose of drawing
    //! Shapes.  When you make your program, start by creating a shader.
    //! Shaders are built at compile time, so you'll need to make a build.rs and
    //! depend on the [`res`](https://crates.io/crates/res) crate.  Calling
    //! `generate()` in your build.rs will generate your shaders.
    //!
    //! ## Shape
    //! A shape is a collection of vertices that when connected make a 2D or 3D
    //! shape.  Shapes can only be used with one Shader because they may have
    //! shader-specific additional information attached to them like color or
    //! graphic coordinates.
    //!
    //! ## Instance
    //! Shapes themselves can't be drawn, first you must make an Instance of the
    //! Shape.  Instances can have position attached to them, and/or rotation
    //! and size.
    //!
    //! # Example
    //! ```rust
    //! // TODO
    //! ```

    include!("internal/barg.rs");

    pub use crate::timer::*;
}

#[cfg(feature = "clock")]
pub mod clock;

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

#[cfg(feature = "graphics")]
#[doc(hidden)]
pub use graphics::*;

#[cfg(feature = "clock")]
#[doc(hidden)]
pub use clock::*;

#[doc(hidden)]
pub use internal::start;
#[doc(hidden)]
pub use run::Loop::*;

pub use internal::delta;

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
macro_rules! init {
    ($home_loop: expr, $init_data: expr) => {
        fn main() {
            let mut window_title = String::new();
            let mut cap = true;

            let fallback = env!("CARGO_PKG_NAME");

            for c in fallback.chars() {
                match c {
                    '.' | '-' | '_' => {
                        window_title.push(' ');
                        cap = true;
                    }
                    a => {
                        if cap {
                            cap = false;
                            for i in a.to_uppercase() {
                                window_title.push(i);
                            }
                        } else {
                            window_title.push(a);
                        }
                    }
                }
            }

            cala::start(window_title.as_str(), $home_loop, &|| $init_data);
        }
    };
}
