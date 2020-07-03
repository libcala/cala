#[cfg(target_arch = "wasm32")]
pub use cala_core::exec;

/// Select an asynchronous function as the entry point for the application.
#[cfg(all(not(target_arch = "wasm32"), not(feature = "graphics")))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        fn main() {
            use $crate::__hidden::Executor;
            static EXECUTOR: $crate::__hidden::CvarExec
                = $crate::__hidden::CvarExec::new();
            EXECUTOR.block_on($main());
        }
    };
}

/// Set an asynchronous function as the entry point for the application.
#[cfg(all(not(target_arch = "wasm32"), feature = "graphics"))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        fn main() {
            std::thread::spawn(|| {
                use $crate::__hidden::Executor;
                static EXECUTOR: $crate::__hidden::CvarExec
                    = $crate::__hidden::CvarExec::new();
                EXECUTOR.block_on($main());
                std::process::exit(0);
            });
            $crate::__hidden::graphics_thread();
        }
    };
}
