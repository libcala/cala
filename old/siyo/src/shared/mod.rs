#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(feature = "speaker", feature = "mic")
))]
pub(crate) mod alsa;

#[cfg(target_arch = "wasm32")]
mod wasm;

// Global library state.
#[doc(hidden)]
pub struct SiyoHiddenGlobal__ {
    #[cfg(feature = "screen")]
    pub screen: Option<crate::screen::screen::App>,

    pub inputm: Option<crate::hid::NativeManager>,
}

#[doc(hidden)]
pub mod siyo_hidden__ {
    pub use crate::hid::private::new as hid_new;
    pub use crate::hid::private::update as hid_update;
}

#[cfg(feature = "screen")]
pub(crate) fn screen() -> &'static mut crate::screen::screen::App {
    unsafe { GLOBAL.screen.as_mut().unwrap() }
}

#[doc(hidden)]
pub static mut SIYO_HIDDEN_GLOBAL__: SiyoHiddenGlobal__ = SiyoHiddenGlobal__ {
    #[cfg(feature = "screen")]
    screen: None,

    inputm: None,
};

/// Create the main function.
///
/// ```
/// extern crate siyo;
///
/// use siyo::*;
///
/// main!(
///     Ctx::new,
///     struct Ctx {
///         mode: fn(app: &mut Ctx) -> bool,
///     }
/// );
///
/// impl Ctx {
///     fn new() -> Ctx {
///         Ctx {
///             mode: mode,
///         }
///     }
///
///     fn run(&mut self) -> bool {
///         (self.mode)(self)
///     }
/// }
///
/// fn mode(app: &mut Ctx) -> bool {
///     // Your code.
///     false // quit right away.
/// }
/// ```
#[macro_export]
macro_rules! main {
	($ctx_type:path, $ctx_def:item) => {
        $ctx_def

        fn main() -> Result<(), String> {
            // Set Global
            unsafe {
                $crate::SIYO_HIDDEN_GLOBAL__ = $crate::SiyoHiddenGlobal__ {
                    #[cfg(feature = "screen")]
                    screen: Some($crate::screen::screen::App::new()),

                    inputm: Some($crate::siyo_hidden__::hid_new()),
                };
            }



            #[cfg(target_arch = "wasm32")]
            {
                // $ctx_type()
                $crate::siyo_web::start(new, run);

//                unsafe {
//                    wasm.event_loop();
//                }
            }

            // Initialize the App Data
//            let mut app = $ctx_type();

            #[cfg(not(target_arch = "wasm32"))]
            while app.run() {
                #[cfg(feature = "screen")]
                {
                    screen().dt = screen().display.update();
                }

                unsafe {
                    $crate::siyo_hidden__::hid_update(
                        $crate::SIYO_HIDDEN_GLOBAL__.inputm.as_mut().unwrap()
                    );
                }
            }

            ::std::process::exit(0)
        }
    }
}
