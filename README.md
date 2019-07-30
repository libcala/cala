![cala](https://raw.githubusercontent.com/Aldarobot/cala/master/logo.png)

### Note
Cala is a complete redesign of previous library [ADI](https://crates.io/crates/adi).  It is still in it's early stages.

# About
Easily create cross-platform applications.  Some common tasks are not easily portable across different platforms, and this crate hopes to fix that.  That way you don't have to worry about how to port your GUI, audio, or bluetooth interface, etc. and can get straight to building your application's content!

Cala is a platform-agnostic system interface for hardware IO.  This means that eventually, Cala should support all of the different hardware that's connected to your computer.  Cala is designed so that it talks to the operating system to interface with the hardware, so no special permissions are needed for your application.

## Features
- Targeted Platforms: Linux (includes Raspberry Pi), MacOS, Redox, Android, Windows, iOS, Web (WASM), Nintendo Switch, XBox, PlayStation, FreeBSD, others (Maybe FreeDOS for fun 😉️).
- Getting user information (Linux, Windows, MacOS)
- Playing / recording audio (Linux)
- Filesystem loading / saving ZIP files (Linux, Windows)
- Game Controller - JoyStick (Linux)
- Clock - Date, Time of day, Timer (All Platforms)
- Graphics - Render and User Interface (Linux)
- Camera - Webcam (NOT IMPLEMENTED YET)
- Hardware acceleration - SIMD, GPU (NOT IMPLEMENTED YET)
- Network - Bluetooth & Wifi Direct (NOT IMPLEMENTED YET)

## Getting Started
- TODO

## Links
- [Website](https://aldarobot.github.io/cala/)
- [Cargo](https://crates.io/crates/cala)
- [Documentation](https://docs.rs/cala)
- [Change Log](https://aldarobot.github.io/cala/CHANGELOG)
- [Contributors](https://aldarobot.github.io/cala/CONTRIBUTORS)
- [Code of Conduct](https://aldarobot.github.io/cala/CODEOFCONDUCT)
- [Join Zulip Chat](https://plopgrizzly.zulipchat.com/join/pp13s6clnexk03tvlnrtjvi1/)
