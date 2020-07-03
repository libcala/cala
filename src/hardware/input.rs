//! **feature:input** - Get user input.
//!
//! # Usage
//! ```rust
//! // TODO
//! ```

#[cfg(feature = "draw")]
use window::input as input_source;

#[cfg(not(feature = "draw"))]
use human as input_source;

pub use self::input_source::{
    input, renumber, rumble, set_mode, GameInput, Input, Mode, TextInput,
    UiInput,
};
