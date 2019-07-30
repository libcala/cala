static START: std::sync::Once = std::sync::ONCE_INIT;
static NANOS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

// // // // // //

/// Initialize Cala.
fn init(_name: &str, _run: fn(nanos: u64)) {
    START.call_once(|| {
        #[cfg(feature = "user")]
        {
            // Initialize user data.
            crate::user::initialize_user_io();
        }
        #[cfg(feature = "controller")]
        {
            // Initialize controller port data.
            crate::controller::initialize_controller_io();
        }
        #[cfg(feature = "audio")]
        {
            // Intialize audio interface.
            crate::audio::initialize_audio_io();
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

/// Get nanoseconds passed since previous frame.
pub fn delta() -> u64 {
    NANOS.load(std::sync::atomic::Ordering::Relaxed)
}

///
pub fn start<T>(
    window_title: &str,
    home_loop: fn(a: &mut T) -> crate::Loop<T>,
    init_data: &Fn() -> T,
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
