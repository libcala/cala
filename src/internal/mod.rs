static START: std::sync::Once = std::sync::Once::new();
static NANOS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

// // // // // //

#[cfg(target_arch = "wasm32")]
use stdweb::js;

/// Initialize Cala.
fn init(_name: &str, _run: fn(nanos: u64)) {
    START.call_once(|| {
        #[cfg(feature = "audio")]
        {
            // Intialize audio interface.
            // crate::audio::initialize_audio_io();
        }
        #[cfg(feature = "graphics")]
        {
            crate::graphics::initialize_video_io(_name, _run);
        }
    });
}

/// Redraw the screen when "video" feature is enabled.
fn r#loop(_run: fn(nanos: u64)) -> bool {
    #[cfg(feature = "graphics")]
    {
        crate::graphics::loop_video_io()
    }
    #[cfg(not(feature = "graphics"))]
    {
        _run(0); // TODO: nanos.
        true
    }
}

/// Log an informative (stdout) message in both release and debug mode.
///
/// Do not overuse this function.  Release builds should have few logs.
#[inline(always)]
pub fn info(string: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        js! {
            console.info(@{string})
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", string);
    }
}

/// Log a warning (stderr) message in both release and debug mode.
///
/// Do not overuse this function.  Release builds should have few logs.
#[inline(always)]
pub fn warn(string: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        js! {
            console.warn(@{string})
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("{}", string);
    }
}

/// Log an informative (stdout) message in only debug mode.
///
/// This is designed to be used for debugging.
#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn note(string: &str) {}

/// Log an informative (stdout) message in only debug mode.
///
/// This is designed to be used for debugging.
#[cfg(debug_assertions)]
#[inline(always)]
pub fn note(string: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        js! {
            console.debug(@{string})
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", string);
    }
}

/// Get nanoseconds passed since previous frame.
pub fn delta() -> u64 {
    NANOS.load(std::sync::atomic::Ordering::Relaxed)
}

///
pub fn start<T>(
    window_title: &str,
    home_loop: fn(a: &mut T) -> crate::Loop<T>,
    init_data: &dyn Fn() -> T,
) {
    use std::ffi::c_void;

    fn dummy(_: *mut c_void) -> crate::Loop<c_void> {
        crate::Exit
    }

    static mut CURRENT_LOOP: fn(*mut c_void) -> crate::Loop<c_void> = dummy;
    static mut DATA: *mut c_void = std::ptr::null_mut();
    static mut RETURN: crate::Loop<c_void> = crate::Continue;

    fn run(nanos: u64) {
        unsafe {
            RETURN = (CURRENT_LOOP)(DATA);
            NANOS.store(nanos, std::sync::atomic::Ordering::Relaxed);
        }
    }

    init(window_title, run);

    let mut current_loops: Vec<fn(&mut T) -> crate::Loop<T>> = vec![home_loop];

    let mut data = { init_data() };

    unsafe {
        DATA = &mut data as *mut _ as *mut _;
    }

    'a: loop {
        unsafe {
            CURRENT_LOOP =
                std::mem::transmute(current_loops[current_loops.len() - 1]);
        }
        if !r#loop(run) {
            break 'a;
        }
        let rtn: crate::Loop<T> = unsafe {
            let mut rtn = crate::Continue;
            std::mem::swap(&mut RETURN, &mut rtn);
            std::mem::transmute(rtn)
        };
        match rtn {
            crate::Exit => {
                break 'a;
            }
            crate::Continue => { /* do nothing */ }
            crate::Back => {
                if current_loops.pop().is_none() {
                    break 'a;
                }
            }
            crate::Replace(loop_a) => {
                if current_loops.pop().is_none() {
                    break 'a;
                }
                current_loops.push(loop_a);
            }
            crate::Append(loop_a) => {
                current_loops.push(loop_a);
            }
            crate::ReplaceWithBack(loop_a, loop_b) => {
                if current_loops.pop().is_none() {
                    break 'a;
                }
                current_loops.push(loop_b);
                current_loops.push(loop_a);
            }
        }
    }
}
