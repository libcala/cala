# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://free.plopgrizzly.com/semver/).

## [Unreleased]
### TODO
- Possibly redesign the controller API to be event based using a message queue.

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
