# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://code.plopgrizzly.com/semver/).

## [Unreleased]
### TODO
- Possibly redesign the controller API to be event based using a message queue.

## [0.7.0] - 2019-11-04
### Added
- Logging macros `info!()`, `warn!()` and `note!()`
- Web Assembly support for `user` feature, and logging.

### Changed
- Updated `whoami`

## [0.6.0] - 2019-10-25
### Added
- `aspect()` for getting aspect ratio

### Fixed
- All of warnings, including clippy warnings

### Changed
- Replaced instances with groups.

## [0.5.0] - 2019-08-02
### Added
- Toolbar
- `draw_graphic()` - for textures.
- `texture_coords()` - for texture atlas.
- `set_camera()` - for camera.
- `key()` - for getting keyboard input.

### Fixed
- `clock` feature not compiling.
- Examples not compiling

## [0.4.0] - 2019-07-07
### Added
- `graphics` feature and support for graphical user interfaces.
- `clock` feature and support for getting date and time.

## [0.3.0] - 2019-05-23
### Added
- `controllers()` which returns a `ControllerIter`.
- `Loop` and `loop_init!()`.  Together they handle the program's control flow.  It also makes it easier in the future to port to Android and other platforms that don't use `main()`.  Besides that, it's a nice abstraction that works similarly to Android activities.
- Multi-threaded support.  You should now be able to do IO calls from multiple threads.

### Removed
- `App` type.

### Changed
- Instead of `App` type everything is in a module, and must be enabled with a feature.  This makes it so you don't have to compile the parts of the project you don't need.

## [0.2.1] - 2019-05-13
### Fixed
- L & R triggers on controllers always returning 0.

## [0.2.0] - 2019-05-12
### Added
- Joystick / controller support with API for emulation (not complete yet).

## [0.1.0] - 2019-05-01
### Added
- Getting user information (Linux, Windows, MacOS).
- Playing / recording audio (Linux).
- Filesystem loading / saving ZIP files (Linux, Windows).
