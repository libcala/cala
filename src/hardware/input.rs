//! **feature:input** - Get user input.
//!
//! # Usage
//! ```rust
//! // TODO
//! ```

#[cfg(feature = "graphics")]
use window::input as input_source;

#[cfg(not(feature = "graphics"))]
use human as input_source;

pub use self::input_source::{GameInput, Input, Mode, TextInput, UiInput, input, renumber, rumble, set_mode};
