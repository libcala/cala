//! **feature:input** - Get user input.
//!
//! # Usage
//! ```rust,no_run
//! todo!();
//! /*use cala::*;
//! use input::{Input, UiInput, GameInput, TextInput};
//!
//! exec!(input);
//! async fn input<'a>() {
//!     loop {
//!         match cala::input::input().await {
//!             Input::Ui(UiInput::Back) => break,
//!             Input::Game(_id, GameInput::Back) => break,
//!             Input::Text(TextInput::Back) => break,
//!             input => println!("{:?}", input),
//!         }
//!     }
//! }*/
//! ```

#[cfg(feature = "draw")]
use window::input as input_source;

#[cfg(not(feature = "draw"))]
use human as input_source;

pub use self::input_source::{
    input, renumber, rumble, set_mode, GameInput, Input, Mode, TextInput,
    UiInput,
};
