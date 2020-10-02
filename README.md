![Cala](https://libcala.github.io/logo.svg)

#### Make portable apps and video games in Rust!

[![docs.rs](https://docs.rs/cala/badge.svg)](https://docs.rs/cala)
[![crates.io](https://img.shields.io/crates/v/cala.svg)](https://crates.io/crates/cala)
[![Zulip Chat](https://img.shields.io/badge/zulip-join_chat-darkgreen.svg)](https://cala.zulipchat.com/join/wkdkw53xb5htnchg8kqz0du0/)

[About](https://libcala.github.io/cala) |
[Source](https://github.com/libcala/cala) |
[Changelog](https://libcala.github.io/cala/changelog) |
[Tutorials](https://libcala.github.io/tutorials) |
[Blog](https://libcala.github.io)

Are you sad that the standard library's only system interface is the filesystem?  This
crate is for you!  This crate provides a safe abstraction over windowing, audio,
accessibility, input, and video.  This crate, however, is not intended to support
multimedia format parsing - that's developed as a separate crate:
[Caved](https://crates.io/crates/caved).

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

### Feature Support
Each system interface can be enabled with a feature.  Names of features match the
module names where the API is located.  Just add it to your Cargo.toml:

```toml
[dependencies.cala]
version = "0.8"
features = ["access", "speaker"]
```

Here's a list of all of the targeted platforms and what they support.
 - ✓: supported
 - —: not planned / possible
 - ?: untested

| Feature           | Linux | MacOS | Windows | Web | Android |
|-------------------|-------|-------|---------|-----|---------|
| [access][100]     |       |       |         |     |         |
| [bluetooth][101]  |       |       |         |     |         |
| [camera][102]     |       |       |         |     |         |
| [draw][103]       | ✓     |[9][3] | [8][6]  |     |         |
| [exec][104]       | ✓     |       |         | ✓   |         |
| [file][105]       | ✓     | ✓     | ✓       |     |         |
| [gpio][106]       |       | —     | —       | —   |         |
| [input][107]      | ✓     |[7][2] | [6][5]  | ?   |         |
| [journal][108]    | ✓     | ✓     | ✓       | ✓   |         |
| [microphone][109] | ✓     |[5][1] | [4][4]  | ✓   | ?       |
| [net][110]        | ✓     | ✓     | ✓       | ?   |         |
| [pixels][111]     | ✓     |[9][3] |         |     |         |
| [speaker][112]    | ✓     |       | [4][4]  | ✓   | ?       |
| [time][113]       | ✓     | ✓     | ✓       | ✓   |         |
| [user][114]       | ✓     | ✓     | ✓       | ✓   |         |

Module documentation may include simple tutorials.  More in depth tutorials may be
found [here](https://libcala.github.io/tutorials).

#### Not Yet Attempted Support, But Planned
- iOS
- Fuchsia
- Redox
- Nintendo Switch
- XBox
- PlayStation
- BSD variants
- Others not on this list that you will make a pull request for adding them

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE](https://github.com/libcala/cala/blob/master/LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Zlib License,
   ([LICENSE-ZLIB](https://github.com/libcala/cala/blob/master/LICENSE-ZLIB) or
   [https://opensource.org/licenses/Zlib](https://opensource.org/licenses/Zlib))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Contributors are always welcome (thank you for being interested!), whether it
be a bug report, bug fix, feature request, feature implementation or whatever.
Don't be shy about getting involved.  I always make time to fix bugs, so usually
a patched version of the library will be out a few days after a report.
Features requests will not complete as fast.  If you have any questions, design
critques, or want me to find you something to work on based on your skill level,
you can email me at [jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).
Otherwise,
[here's a link to the issues on GitHub](https://github.com/libcala/cala/issues).
Before contributing, check out the
[contribution guidelines](https://github.com/libcala/cala/blob/master/CONTRIBUTING.md),
and, as always, make sure to follow the
[code of conduct](https://github.com/libcala/cala/blob/master/CODE_OF_CONDUCT.md).

[1]: https://github.com/libcala/cala/issues/5
[2]: https://github.com/libcala/cala/issues/7
[3]: https://github.com/libcala/cala/issues/9
[4]: https://github.com/libcala/cala/issues/4
[5]: https://github.com/libcala/cala/issues/6
[6]: https://github.com/libcala/cala/issues/8
[100]: https://docs.rs/cala/latest/cala/accel/
[101]: https://docs.rs/cala/latest/cala/bluetooth/
[102]: https://docs.rs/cala/latest/cala/camera/
[103]: https://docs.rs/cala/latest/cala/draw/
[104]: https://docs.rs/cala/latest/cala/exec/
[105]: https://docs.rs/cala/latest/cala/file/
[106]: https://docs.rs/cala/latest/cala/gpio/
[107]: https://docs.rs/cala/latest/cala/input/
[108]: https://docs.rs/cala/latest/cala/journal/
[109]: https://docs.rs/cala/latest/cala/microphone/
[110]: https://docs.rs/cala/latest/cala/net/
[111]: https://docs.rs/cala/latest/cala/pixels/
[112]: https://docs.rs/cala/latest/cala/speaker/
[113]: https://docs.rs/cala/latest/cala/time/
[114]: https://docs.rs/cala/latest/cala/user/
