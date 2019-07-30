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
//! ## Getting Started
//! * TODO

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://jeronaldaron.github.io/cala/icon.svg",
    html_favicon_url = "https://jeronaldaron.github.io/cala/icon.svg"
)]

mod audio;
mod controller;
mod dive;
mod user;

pub use audio::AudioSample;
pub use controller::ControllerLayout;
pub use dive::App;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
