![Cala][0]

#### Make portable apps and video games in Rust!
[![docs.rs][1]][10] [![crates.io][2]][11] [![Zulip Chat][3]][12]

[About][13] | [Source][14] | [Changelog][15] | [Tutorials][16] | [Blog][17]

Are you sad that the standard library's only system interface is the filesystem?
This crate is for you!  This crate provides a safe abstraction over windowing,
audio, accessibility, input, and video.  This crate, however, is not intended to
support multimedia format parsing - that's developed as a separate crate:
[Caved][20].

Cala is intended to be an "oxidized re-implementation" of both
[Flutter][21]/[GTK][22] and [SDL][23]/[other SDL projects][24]
in one library!  Flutter is mostly intended for mobile applications, and GTK is
just for desktop applications, but what if you want to develop the same app for
both?  Then you use this crate (a lot of features are still WIP)!  Cala
additionally targets the web and bare metal systems.  Note also that even if
you're not trying to make your application / video game extremely portable, you
can still use this crate!

You might ask, "Shouldn't apps and video games use separate libararies; Why are
they combined?".  They usually need do the same thing, and some desktop
application depend on SDL, like [VLC][25], and some
video games depend on GTK, like [Veloren][26] (at least when
built on Linux).  There's clearly a shared interest; so they *should* be
combined.  That said, Cala is extremely modular, and doesn't include any modules
at all unless you enable some features.  The modules are named exactly the same
as the features, so you enable the `camera` feature to be able to use the
`camera` module.

### Naming
The name cala is derived from the fungus known as [calafate rust][30].

### Feature Support
Each system interface can be enabled with a feature.  Names of features match
the module names where the API is located.  Just add it to your Cargo.toml:

```toml
[dependencies.cala]
version = "0.8"
features = ["access", "speaker"]
```

Here's a list of all of the targeted platforms and what they support.
 - ✓: supported
 - —: not planned / possible
 - ?: untested

| Feature           | Linux | MacOS  | Windows | Web | Android |
|-------------------|-------|--------|---------|-----|---------|
| [access][100]     |       |        |         |     |         |
| [bluetooth][101]  |       |        |         |     |         |
| [camera][102]     |       |        |         |     |         |
| [draw][103]       | ✓     |[9][52] | [8][55] |     |         |
| [exec][104]       | ✓     |        |         | ✓   |         |
| [file][105]       | ✓     | ✓      | ✓       |     |         |
| [gpio][106]       |       | —      | —       | —   |         |
| [input][107]      | ✓     |[7][51] | [6][54] | ?   |         |
| [log][108]        | ✓     | ✓      | ✓       | ✓   |         |
| [microphone][109] | ✓     |[5][50] | [4][53] | ✓   | ?       |
| [net][110]        | ✓     | ✓      | ✓       | ?   |         |
| [pixels][111]     | ✓     |[9][52] |         |     |         |
| [speaker][112]    | ✓     |        | [4][53] | ✓   | ?       |
| [time][113]       | ✓     | ✓      | ✓       | ✓   |         |
| [user][114]       | ✓     | ✓      | ✓       | ✓   |         |

Module documentation may include simple tutorials.  More in depth tutorials may
be found [here][16].

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
 - Apache License, Version 2.0, ([LICENSE-APACHE][40]
   or [https://www.apache.org/licenses/LICENSE-2.0][41])
 - Zlib License, ([LICENSE-ZLIB][42]
   or [https://opensource.org/licenses/Zlib][43])

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
you can email me at [jeronlau@plopgrizzly.com][99].  Otherwise,
[here's a link to the issues on GitHub][44].  Before contributing, check out the
[contribution guidelines][45], and, as always, make sure to follow the
[code of conduct][46].

[0]: https://libcala.github.io/logo.svg
[1]: https://docs.rs/cala/badge.svg
[2]: https://img.shields.io/crates/v/cala.svg
[3]: https://img.shields.io/badge/zulip-join_chat-darkgreen.svg

[10]: https://docs.rs/cala
[11]: https://crates.io/crates/cala
[12]: https://cala.zulipchat.com/join/wkdkw53xb5htnchg8kqz0du0
[13]: https://libcala.github.io/cala
[14]: https://github.com/libcala/cala
[15]: https://libcala.github.io/cala/changelog
[16]: https://libcala.github.io/tutorials
[17]: https://libcala.github.io

[20]: https://crates.io/crates/caved
[21]: https://flutter.dev
[22]: https://www.gtk.org
[23]: https://www.libsdl.org
[24]: https://www.libsdl.org/projects
[25]: https://www.videolan.org/vlc
[26]: https://veloren.net

[30]: https://en.wikipedia.org/wiki/Aecidium_magellanicum

[40]: https://github.com/libcala/cala/blob/master/LICENSE-APACHE
[41]: https://www.apache.org/licenses/LICENSE-2.0
[42]: https://github.com/libcala/cala/blob/master/LICENSE-ZLIB
[43]: https://opensource.org/licenses/Zlib
[44]: https://github.com/libcala/cala/issues
[45]: https://github.com/libcala/cala/blob/master/CONTRIBUTING.md
[46]: https://github.com/libcala/cala/blob/master/CODE_OF_CONDUCT.md

[50]: https://github.com/libcala/cala/issues/5
[51]: https://github.com/libcala/cala/issues/7
[52]: https://github.com/libcala/cala/issues/9
[53]: https://github.com/libcala/cala/issues/4
[54]: https://github.com/libcala/cala/issues/6
[55]: https://github.com/libcala/cala/issues/8

[99]: mailto:jeronlau@plopgrizzly.com

[100]: https://docs.rs/cala/latest/cala/access
[101]: https://docs.rs/cala/latest/cala/bluetooth
[102]: https://docs.rs/cala/latest/cala/camera
[103]: https://docs.rs/cala/latest/cala/draw
[104]: https://docs.rs/cala/latest/cala/exec
[105]: https://docs.rs/cala/latest/cala/file
[106]: https://docs.rs/cala/latest/cala/gpio
[107]: https://docs.rs/cala/latest/cala/input
[108]: https://docs.rs/cala/latest/cala/log
[109]: https://docs.rs/cala/latest/cala/microphone
[110]: https://docs.rs/cala/latest/cala/net
[111]: https://docs.rs/cala/latest/cala/pixels
[112]: https://docs.rs/cala/latest/cala/speaker
[113]: https://docs.rs/cala/latest/cala/time
[114]: https://docs.rs/cala/latest/cala/user
