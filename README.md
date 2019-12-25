![Cala](https://libcala.github.io/logo.svg)

[![docs.rs](https://docs.rs/cala/badge.svg)](https://docs.rs/cala)
[![build status](https://api.travis-ci.com/libcala/cala.svg?branch=master)](https://travis-ci.com/libcala/cala)
[![crates.io](https://img.shields.io/crates/v/cala.svg)](https://crates.io/crates/cala)
[![Zulip Chat](https://img.shields.io/badge/zulip-join_chat-darkgreen.svg)](https://cala.zulipchat.com/join/wkdkw53xb5htnchg8kqz0du0/)

[About](https://libcala.github.io/cala) |
[Source](https://github.com/libcala/cala) |
[Changelog](https://libcala.github.io/cala/changelog) |
[Tutorials](https://libcala.github.io/tutorials) |
[Blog](https://libcala.github.io)

### Note
Cala is a complete redesign of previous library [ADI]("https://crates.io/crates/adi").  It is still in it's early stages.

# About
Easily create cross-platform applications.  Some common tasks are not easily portable across different platforms, and this crate hopes to fix that.  That way you don't have to worry about how to port your GUI, audio, or bluetooth interface, etc. and can get straight to building your application's content!

Cala is a platform-agnostic system interface for hardware IO.  This means that eventually, Cala should support all of the different hardware that's connected to your computer.  Cala is designed so that it talks to the operating system to interface with the hardware, so no special permissions are needed for your application.  Here's a list of all of the targeted platforms (**bold** means a port has been made, *italic* means the feature doesn't work on the platform):

- **Linux**
- **MacOS** - missing [*audio*](https://github.com/libcala/cala/issues/5), [*controller*](https://github.com/libcala/cala/issues/7), [*graphics*](https://github.com/libcala/cala/issues/9)
- **Windows** - missing [*audio*](https://github.com/libcala/cala/issues/4), [*controller*](https://github.com/libcala/cala/issues/6), [*graphics*](https://github.com/libcala/cala/issues/8)
- **Web (WASM)** - missing audio, controller, graphics, files
- Redox
- Android
- iOS
- Nintendo Switch
- XBox
- PlayStation
- FreeBSD
- Maybe FreeDOS for fun 😉️
- Others not on this list that you will make a pull request for adding them

# Motivation & Naming
The aim is to create a newer, better GTK + SDL in Rust!  Why GTK + SDL?  Because a lot of programs need to depend on both anyway (like [totem](https://en.wikipedia.org/wiki/Totem_Video_Player)), and they do a lot of the same things; Usually one library does each specific task better than the other.  The goal of this library is to provide the common ground for video games and general GUI applications together.  The name cala is derived from the fungus known as calafate rust.

# Getting Started
Each hardware interface can be enabled with a feature.  For example, If you
want to depend on the `audio` feature and the `clock`
feature, you might put this in your `Cargo.toml`:

<!--
```toml
[dependencies.cala]
version = "0.5"
features = ["audio", "clock"]
```
-->

<p style="width:100%"><pre lang="toml"><code><span style="color:#FFF;font-weight:bold;">[dependencies.cala]</span>
<span style="color:#0F0;font-weight:bold;">version</span> = <span style="color:#0F0">"0.5"</span>
<span style="color:#0F0;font-weight:bold;">features</span> = [<span style="color:#0F0">"audio"</span>, <span style="color:#0F0">"clock"</span>]</code></pre></p>

There is a module for each feature (feature and module names match).  Module documentation may include simple tutorials.  More in depth tutorials may be
found [here](https://libcala.github.io/tutorials).

## Features
Here's a list of the features, with links to documentation.

- [`user`](https://docs.rs/cala/0.5.0/cala/user/index.html) - Getting user information
- [`audio`](https://docs.rs/cala/0.5.0/cala/audio/index.html) - Playing / recording audio
- [`clock`](https://docs.rs/cala/0.5.0/cala/clock/index.html) - Date, Time of day, Timer
- [`controller`](https://docs.rs/cala/0.5.0/cala/controller/index.html) - Game Controller - JoyStick
- [`files`](https://docs.rs/cala/0.5.0/cala/files/index.html) - Containerized filesystem loading / saving ZIP files
- [`graphics`](https://docs.rs/cala/0.5.0/cala/graphics/index.html) - Render and User Interface
- Camera - Webcam [unimplemented](https://github.com/libcala/cala/issues/1)
- Network - Bluetooth & Wifi Direct [unimplemented](https://github.com/libcala/cala/issues/10)
- Hardware acceleration - SIMD, GPU [unimplemented](https://github.com/libcala/cala/issues/11)

# Contributing
Contributors are always welcome!  Whether it is a bug report, bug fix, feature
request, feature implementation or whatever.  Don't be shy about getting
involved.  I always make time to fix bugs, so usually a patched version of the
library will be out soon after a report.  Features take me longer, though.  I'll
also always listen to any design critiques you have.  If you have any questions
you can email me at jeronlau@plopgrizzly.com.  Otherwise, here's a link to the
[issues on GitHub](https://github.com/libcala/cala/issues).

And, as always, make sure to always follow the
[code of conduct](https://github.com/libcala/cala/blob/master/CODEOFCONDUCT.md).
Happy coding!

# License
This repository is licensed under either of the following:

- MIT License (MIT) - See accompanying file
  [LICENSE_MIT.txt](https://github.com/libcala/cala/blob/master/LICENSE_MIT.txt)
  or copy at https://opensource.org/licenses/MIT
- Boost Software License (BSL-1.0) - See accompanying file
  [LICENSE_BSL.txt](https://github.com/libcala/cala/blob/master/LICENSE_BSL.txt)
  or copy at https://www.boost.org/LICENSE_1_0.txt

at your option.

## Contribution Licensing
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above without any
additional terms or conditions.
