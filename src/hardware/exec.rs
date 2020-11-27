//! **feature:exec** - Single / multi-processor execution of tasks / threads.
//!
//! # Getting Started
//! ```
//! use cala::*;
//!
//! exec!(exec);
//! async fn exec() {
//!     todo!();
//! }
//! ```

#[cfg(target_arch = "wasm32")]
/// **feature:exec** -
pub use cala_core::exec;

/// **feature:exec** - Select an asynchronous function as the entry point for
/// the application.
#[cfg(all(not(target_arch = "wasm32"), not(feature = "draw")))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        fn main() {
            $crate::__hidden::exec!($main());
        }
    };
}

/// **feature:exec** - Set an asynchronous function as the entry point for the
/// application.
#[cfg(all(not(target_arch = "wasm32"), feature = "draw"))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        fn main() {
            std::thread::spawn(|| {
                $crate::__hidden::exec!($main());
                std::process::exit(0);
            });
            $crate::__hidden::draw_thread();
        }
    };
}
