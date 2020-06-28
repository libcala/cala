![Cala](https://libcala.github.io/logo.svg)

#### Make portable apps and video games in Rust!

[![docs.rs](https://docs.rs/cala/badge.svg)](https://docs.rs/cala)
[![build status](https://api.travis-ci.com/libcala/cala.svg?branch=master)](https://travis-ci.com/libcala/cala)
[![crates.io](https://img.shields.io/crates/v/cala.svg)](https://crates.io/crates/cala)
[![Zulip Chat](https://img.shields.io/badge/zulip-join_chat-darkgreen.svg)](https://cala.zulipchat.com/join/wkdkw53xb5htnchg8kqz0du0/)

[About](https://libcala.github.io/cala) |
[Source](https://github.com/libcala/cala) |
[Changelog](https://libcala.github.io/cala/changelog) |
[Tutorials](https://libcala.github.io/tutorials) |
[Blog](https://libcala.github.io)

Cala is intended to be an "oxidized re-implementation" of both
[Flutter](https://flutter.dev/)/[GTK](https://www.gtk.org/) and
[SDL](https://www.libsdl.org/)/[other SDL projects](https://www.libsdl.org/projects/)
in one library!  Flutter is mostly intended for mobile applications, and GTK is
just for desktop applications, but what if you want to develop the same app for
both?  Then you use this crate (a lot of features are still WIP)!  Cala
additionally targets the web and bare metal systems.  Note also that even if
you're not trying to make your application / video game extremely portable, you
can still use this crate!

You might ask, "Shouldn't apps and video games use separate libararies; Why are
they combined?".  They usually need do the same thing, and some desktop
application depend on SDL, like [VLC](https://www.videolan.org/vlc/), and some
video games depend on GTK, like [Veloren](https://veloren.net/) (at least when
built on Linux).  There's clearly a shared interest; so they *should* be
combined.  That said, Cala is extremely modular, and doesn't include any modules
at all unless you enable some features.  The modules are named exactly the same
as the features, so you enable the `audio` feature to be able to use the `audio`
module.

### Naming
The name cala is derived from the fungus known as
[calafate rust](https://en.wikipedia.org/wiki/Aecidium_magellanicum).

### Support
Here's a list of all of the targeted platforms (**bold** means a port has been made, *italic* means the feature doesn't work on the platform):

- **Linux**
- **MacOS** - WIP [*audio*](https://github.com/libcala/cala/issues/5), [*controller*](https://github.com/libcala/cala/issues/7), [*graphics*](https://github.com/libcala/cala/issues/9)
- **Windows** - WIP [*audio*](https://github.com/libcala/cala/issues/4), [*controller*](https://github.com/libcala/cala/issues/6), [*graphics*](https://github.com/libcala/cala/issues/8)
- **Web (WASM)** - WIP controller, graphics, files
- Android
- iOS
- Fuchsia
- Redox
- Nintendo Switch
- XBox
- PlayStation
- BSD variants
- Others not on this list that you will make a pull request for adding them

# Getting Started
Each hardware interface can be enabled with a feature.  For example, If you
want to depend on the `audio` feature and the `clock`
feature, you might put this in your `Cargo.toml`:

```toml
[dependencies.cala]
version = "0.8"
features = ["audio", "clock"]
```

There is a module for each feature (feature and module names match).  Module documentation may include simple tutorials.  More in depth tutorials may be
found [here](https://libcala.github.io/tutorials).

## Features
Here's a list of the features, with links to documentation.

- [`user`](https://docs.rs/cala/0.7.0/cala/user/index.html) - Getting user information
- [`audio`](https://docs.rs/cala/0.7.0/cala/audio/index.html) - Playing / recording audio
- [`clock`](https://docs.rs/cala/0.7.0/cala/clock/index.html) - Date, Time of day, Timer
- [`gamepad`](https://docs.rs/cala/0.7.0/cala/gamepad/index.html) - Game Controller - JoyStick
- [`files`](https://docs.rs/cala/0.7.0/cala/files/index.html) - Containerized filesystem loading / saving ZIP files
- [`graphics`](https://docs.rs/cala/0.7.0/cala/graphics/index.html) - Render and User Interface
- Camera - Webcam [WIP](https://github.com/libcala/cala/issues/1)
- Network - Bluetooth & Wifi Direct [WIP](https://github.com/libcala/cala/issues/10)
- Hardware acceleration - SIMD, GPU [WIP](https://github.com/libcala/cala/issues/11)

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
