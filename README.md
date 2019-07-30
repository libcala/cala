<p align="center">
  <img alt="Cala" src="https://aldarobot.plopgrizzly.com/cala/logo.svg">
</p>
<p align="center">
  <a href="https://docs.rs/cala">
    <img src="https://docs.rs/cala/badge.svg">
  </a>
  <a href="https://travis-ci.com/Aldarobot/cala">
      <img src="https://api.travis-ci.com/Aldarobot/cala.svg?branch=stable" alt="Cala Build Status">
  </a>
  <a href="https://crates.io/crates/cala">
      <img src="http://img.shields.io/crates/v/cala.svg?label=cala" alt = "cala on crates.io">
  </a>
  <a href="https://discord.gg/nXwF59K">
    <img src="https://img.shields.io/badge/discord-join%20server-green.svg?style=flat-square" alt="Discord">
  </a>
  <br>
  <strong><a href="https://aldarobot.github.io/cala/">Website</a> | <a href="https://aldarobot.github.io/cala/CHANGELOG">Changelog</a> | <a href=https://aldarobot.github.io/cala/CONTRIBUTORS>Contributors</a> | <a href=https://aldarobot.github.io/cala/CODEOFCONDUCT>Code of Conduct</a> </strong>
</p>

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
