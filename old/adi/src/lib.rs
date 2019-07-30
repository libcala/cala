//! # Aldaron's Device Interface.
//! This crate is meant to be used in a build.rs for a [siyo](https://crates.io/crates/siyo)
//! project.  It handles all of the platform-specific control functions, and creates your main
//! function with two parameters, `new` and `run`.  `new` is called when the app opens, and `run` is
//! called every frame.
//! 
//! ## Example
//! ```rust
//! // build.rs
//!
//! fn main() {
//!     adi::gen();
//! }
//! ```
//! 
//! ```
//! // main.rs
//!
//! include!(concat!(env!("OUT_DIR"), "/siyo_app.rs"));
//! 
//! pub struct App {
//!     r#loop: fn run(app: &mut App),
//! }
//!
//! fn new() -> App {
//!     App {
//!         r#loop: main_loop,
//!     }
//! }
//! 
//! fn run(app: &mut App) {
//!     (app.r#loop)();
//! }
//! 
//! fn main_loop() {
//!     
//! }
//! ```

use webgl_generator::*;
use std::fs::File;
use std::path::*;
use std::{fs, io::Write};
use build_helper;
use build_helper::target;

/// Generate the code for the app.
pub fn gen() {
    let triple = target::triple();
    let arch = triple.arch();
    let os = triple.os();
    let out = build_helper::out_dir().to_str().unwrap().to_string();
    let app = format!("{}/{}", out, "siyo_app.rs");

    // Check for arch first.
    if arch == "wasm32" {
        println!("WebGL prepare....");
        let mut file = File::create(&Path::new(&out).join("siyo_webgl.rs")).unwrap();
        Registry::new(Api::WebGl2, Exts::NONE)
            .write_bindings(StdwebGenerator, &mut file)
            .expect("Couldn't build WebGL");
        println!("Compiling for WebAssembly.");
        let mut file = fs::File::create(&app).expect(&format!("Couldn't build app @{}", app));
        file.write_all(include_bytes!("wasm.rs")).unwrap();
    // Check for OS next
    } else if os == "android" {
        println!("Compiling for Android.");
        let mut file = fs::File::create(app).unwrap();
        file.write_all(include_bytes!("android.rs")).unwrap();
    } else if os == "windows" {
        println!("Compiling for Windows.");
        let mut file = fs::File::create(app).unwrap();
        file.write_all(include_bytes!("windows.rs")).unwrap();
    } else if os == "macos" {
        println!("Compiling for MacOS.");
        let mut file = fs::File::create(app).unwrap();
        file.write_all(include_bytes!("macos.rs")).unwrap();
    } else if os == "ios" {
        println!("Compiling for iOS.");
        let mut file = fs::File::create(app).unwrap();
        file.write_all(include_bytes!("ios.rs")).unwrap();
    // Fallback on XCB for Linux/BSD and others.
    } else {
        println!("Compiling for Unix (XCB targets).");
        let mut file = fs::File::create(app).unwrap();
        file.write_all(include_bytes!("unix.rs")).unwrap();
    }
}
