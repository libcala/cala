//! <p align="center">
//! <img src="https://dive.ga/dive/icon.svg" alt="Dive" width="256px" height="256px">
//! </p>
//! 
//! ### Note
//! Dive is a complete redesign of previous library [ADI](https://crates.io/crates/adi).  It is still in it's early stages.
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
//! * Hardware acceleration - SIMD, GPU (NOT IMPLEMENTED YET)
//! * Clock - Date, Time of day, Timer (NOT IMPLEMENTED YET)
//! * GUI - Render, Mouse & Keyboard (NOT IMPLEMENTED YET)
//! * Game Controller - JoyStick (NOT IMPLEMENTED YET)
//! * Camera - Webcam (NOT IMPLEMENTED YET)
//! * Network - Bluetooth & Wifi Direct (NOT IMPLEMENTED YET)
//! 
//! ## Getting Started
//! * TODO

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://dive.ga/dive/icon.svg",
    html_favicon_url = "https://dive.ga/dive/icon.svg"
)]

mod user;
mod dive;
mod audio;

pub use dive::App;
pub use audio::AudioSample;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
