// Copyright Jeron A. Lau 2017-2019.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

extern crate png;
extern crate footile;
extern crate cgmath;

use footile::PixFmt;

mod src;
use adi::*;

use std::ffi::c_void;
use std::ptr::null_mut;

// TODO: Move

mod adi {
    /// Input
    pub enum InputMsg {
        /// Push(x, y) - User has pushed and released.
        /// * x: x position 0-WIDTH
        /// * y: y position 0-AR
        /// * h: horizontal movement since press (pixels)
        /// * v: vertical movement since press (pixels)
        Push(f32, f32, f32, f32),
        /// Drag(x, y, h, v) - User is holding down.
        /// * x: current x position 0-WIDTH
        /// * y: current y position 0-AR
        /// * h: horizontal movement since press (pixels)
        /// * v: vertical movement since press (pixels)
        Drag(f32, f32, f32, f32),
        /// User types a character.
        Text(char),
    }

    /// Input Mode.
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum InputMode {
        /// GUI Mode (This is the default mode).
        Gui,
        /// Typing / Keyboard Mode.
        Text,
        /// Game Mode.
        Game,
    }

/*    /// A bitmap image.
    #[derive(Clone, Debug)]
    pub struct Bitmap(pub usize, pub Vec<u8>);

    impl Bitmap {
        /// Create a new bitmap.
        pub fn new(width: usize, pixels: Vec<u8>) -> Bitmap {
            Bitmap(width, pixels)
        }
    }*/

    /// Output
    #[derive(Clone, Debug)]
    pub enum OutputMsg<'a> {
        /// Change Input Mode
        InputMode(InputMode),
        /// Set Page Gui
        Page(fn(usize) -> &'a [OutputMsg<'a>]),
        /// Set Menu Gui
        Menu(fn(usize) -> &'a [OutputMsg<'a>]),
        /// Draw a Bitmap image (x1, y1, x2, y2, width, pixels)
        DrawBitmap([[f32; 3]; 4], u32),
        ///
        Clear([u8; 4]),
        /// Set Fill Color
        Fill([u8; 4]),
        /// Set Stroke Color
        Stroke([u8; 4]),
        /// Set Stroke Width
        Width(f32),
        /// Matrix Transform.
        Matrix([f32; 16]),
        /// Move X, Y, Z
        Move(f32, f32, f32),
        /// Straight Line X, Y, Z
        Line(f32, f32, f32),
        /// Quadratic bézier curve CX, CY, CZ, X, Y, Z
        Quad((f32, f32, f32), f32, f32, f32),
        /// Cubic bézier curve AX, AY, AZ, BX, BY, BZ, X, Y, Z
        Cube((f32, f32, f32), (f32, f32, f32), f32, f32, f32),
        ///
        Close(),
        /// Process sent data all at once.
        Sync(),
        /// 
        Text(&'a str),
        /// Loading a Bitmap: Bitmap ID, Width, Pixels
        LoadBitmap(u32, u32, Vec<u8>),
    }

    /// System Iterface.
    pub struct Sys<'a, T> {
        pub(super) output_sender: std::sync::mpsc::Sender<(bool, Vec<OutputMsg<'a>>)>,
        pub(super) input_receiver: std::sync::mpsc::Receiver<InputMsg>,
        /// This is your application's context.
        pub app: T,
        /// This is your application's runner.
        pub run: fn(&mut Sys<T>),
    }

    impl<'a, T> Sys<'a, T> {
        /// Send a message to program output.
        pub fn send(&self, msg: Vec<OutputMsg<'a>>) {
            if self.output_sender.send((true, msg)).is_err() {
                std::process::exit(0);
            }
        }

        /// Receive an input message if one exists.
        pub fn recv(&self) -> Option<InputMsg> {
            if let Ok(input) = self.input_receiver.try_recv() {
                Some(input)
            } else {
                None
            }
        }
    }

    pub fn perspective(/*fovy: f32, */w: f32, near: f32, far: f32) -> cgmath::Matrix4<f32> {
//        let f = fovy / 2.0;
        let distance = near - far;

        let c2r2 = (far + near) / distance;
        let c3r2 = (2.0 * far * near) / distance;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        cgmath::Matrix4::new(
            w,   0.0, 0.0,   0.0,
            0.0, w,   0.0,   0.0,
            0.0, 0.0, c2r2, -1.0,
            0.0, 0.0, c3r2,  1.0,
        )
    }
}

fn user_thread<'a>(
    output_sender: std::sync::mpsc::Sender<(bool, Vec<OutputMsg<'a>>)>,
    input_receiver: std::sync::mpsc::Receiver<InputMsg>,
) {
    let mut sys = Sys {
        output_sender,
        input_receiver,
        app: src::APP_INIT,
        run: src::run_init,
    };

    loop {
        (sys.run)(&mut sys);
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Window {
    // Keyboard (XKB)
    keymap: *mut c_void,
    context: *mut c_void,
    state: *mut c_void,
    compose: *mut c_void,
    // Window (XCB)
    pub(crate) window: u32,
    pub(crate) connection: *mut c_void,
    plotter: footile::Plotter,
    raster: footile::RasterB<footile::Rgba8>,
    pixels: Vec<u8>,
    mode: InputMode,
}

impl Window {
    pub fn new(v: Option<i32>) -> Self {
        let connection = xcb_connect2();
        let mut screen = xcb_screen(connection);
        let window = xcb_window(connection, &mut screen, v);
        let (state, keymap, context, compose) = xkb_keyboard();
        let wh = (screen.width_in_pixels, screen.height_in_pixels);
        let pixels = vec![0; wh.0 as usize * wh.1 as usize * 4];
        let mode = InputMode::Gui;
        let plotter = footile::Plotter::new(wh.0 as u32, wh.1 as u32);
        let raster = footile::RasterB::new(plotter.width(), plotter.height());

        Window {
            keymap,
            context,
            state,
            compose,
            window,
            connection,
            pixels,
            mode,
            plotter,
            raster,
        }
    }

    pub(crate) fn update(&mut self, input_sender: &std::sync::mpsc::Sender<InputMsg>) -> Internal {
        let mut rtn = Internal::None;
        unsafe { xcb_flush(self.connection) };
        while xcb_poll_for_event2(
            self.connection,
            self.state,
            self.compose,
            &mut self.plotter,
            &mut self.raster,
            &mut rtn,
            &mut self.mode,
            input_sender,
        ) {}
        rtn
    }
}

enum Internal {
    None,
    Quit,
    Resize,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            xkb_state_unref(self.state);
            xkb_keymap_unref(self.keymap);
            xkb_context_unref(self.context);
            xcb_destroy_window(self.connection, self.window);
            xcb_disconnect(self.connection);
        }
    }
}

#[link(name = "xcb")]
extern "C" {
    fn xcb_poll_for_event(a: *mut c_void) -> *mut XcbGenericEvent;
    fn xcb_flush(a: *mut c_void) -> i32;
    fn xcb_intern_atom(a: *mut c_void, b: u8, c: u16, d: *const u8) -> u32;
    fn xcb_intern_atom_reply(a: *mut c_void, b: u32, c: *mut c_void) -> *mut XcbInternAtomReply;
    fn xcb_change_property(
        a: *mut c_void,
        b: u8,
        c: u32,
        d: u32,
        e: u32,
        f: u8,
        g: u32,
        h: *const c_void,
    ) -> u32;
    fn xcb_map_window(a: *mut c_void, b: u32) -> u32;
    fn xcb_get_setup(a: *mut c_void) -> *mut c_void;
    fn xcb_setup_roots_iterator(a: *mut c_void) -> XcbScreenIterator;
    fn xcb_generate_id(a: *mut c_void) -> u32;
    fn xcb_create_window(
        a: *mut c_void,
        b: u8,
        c: u32,
        d: u32,
        e: i16,
        f: i16,
        g: u16,
        h: u16,
        i: u16,
        j: u16,
        k: u32,
        l: u32,
        m: *mut u32,
    ) -> u32;
    fn xcb_connect(a: *mut c_void, b: *mut c_void) -> *mut c_void;
    fn xcb_destroy_window(a: *mut c_void, b: u32) -> u32;
    fn xcb_disconnect(a: *mut c_void) -> ();
}

#[link(name = "xkbcommon")]
extern "C" {
    fn xkb_context_unref(a: *mut c_void) -> ();
    fn xkb_keymap_unref(a: *mut c_void) -> ();
    fn xkb_state_unref(a: *mut c_void) -> ();
    fn xkb_state_key_get_utf8(a: *mut c_void, b: u32, c: *mut u8, d: usize) -> i32;
    fn xkb_state_update_key(a: *mut c_void, b: u32, c: KeyDirection) -> StateComponent;
    fn xkb_state_new(a: *mut c_void) -> *mut c_void;
    fn xkb_keymap_new_from_string(
        a: *mut c_void,
        b: *const i8,
        c: u32,
        d: CompileFlags,
    ) -> *mut c_void;
    fn xkb_context_new(a: ContextFlags) -> *mut c_void;
    fn xkb_compose_table_new_from_locale(a: *mut c_void, b: *const i8, c: u32) -> *mut c_void;
    fn xkb_compose_state_new(a: *mut c_void, b: u32) -> *mut c_void;
    fn xkb_compose_state_get_utf8(a: *mut c_void, b: *mut u8, c: usize) -> i32;
    fn xkb_compose_state_get_status(a: *mut c_void) -> XkbComposeStatus;
    fn xkb_compose_state_feed(a: *mut c_void, b: u32) -> u32;
    fn xkb_state_key_get_one_sym(a: *mut c_void, b: u32) -> u32;
}

#[allow(dead_code)]
#[repr(C)]
enum XkbComposeStatus {
    Nothing,
    Composing,
    Composed,
    Cancelled,
}

#[allow(dead_code)]
#[repr(C)]
enum StateComponent {
    None,
}

#[repr(C)]
enum KeyDirection {
    Up,
    Down,
}

#[repr(C)]
struct XcbInternAtomReply {
    response_type: u8,
    pad0: u8,
    sequence: u16,
    length: u32,
    atom: u32,
}

#[repr(C)]
enum CompileFlags {
    NoFlags = 0,
}

#[repr(C)]
enum ContextFlags {
    NoFlags = 0,
}

#[repr(C)]
#[derive(Clone)]
struct XcbScreen {
    root: u32,
    default_colormap: u32,
    white_pixel: u32,
    black_pixel: u32,
    current_input_masks: u32,
    width_in_pixels: u16,
    height_in_pixels: u16,
    width_in_millimeters: u16,
    height_in_millimeters: u16,
    min_installed_maps: u16,
    max_installed_maps: u16,
    root_visual: u32,
    backing_stores: u8,
    save_unders: u8,
    root_depth: u8,
    allowed_depths_len: u8,
}

#[repr(C)]
struct XcbScreenIterator {
    data: *mut XcbScreen,
    rem: i32,
    index: i32,
}

#[repr(C)]
#[derive(Clone)]
struct XcbGenericEvent {
    response_type: u8,
    detail: u8,
    sequence: u16,
    timestamp: u32,
    root: u32,
    event: u32,
    child: u32,
    root_x: i16,
    root_y: i16,
    event_x: i16,
    event_y: i16,
    state: u16,
    same_screen: u8,
    pad0: u8,
}

fn xcb_connect2() -> *mut c_void {
    let connection = unsafe { xcb_connect(null_mut(), null_mut()) };
    if connection.is_null() {
        eprintln!("ERROR: XCB couldn't connect to X server, aborting...");
        ::std::process::abort();
    }
    connection
}

fn xcb_screen(connection: *mut c_void) -> XcbScreen {
    let setup = unsafe { xcb_get_setup(connection) };
    unsafe { (*(xcb_setup_roots_iterator(setup).data)).clone() }
}

fn xcb_window(connection: *mut c_void, screen: &mut XcbScreen, v: Option<i32>) -> u32 {
    let window = unsafe { xcb_generate_id(connection) };
    let mut value_list = [0b01000100000000001101111];
    if let Some(v) = v {
        screen.root_visual = unsafe { ::std::mem::transmute(v) };
    }

    unsafe {
        // Create The Window.
        xcb_create_window(
            connection,
            0,
            window,
            screen.root,
            0,
            0,
            screen.width_in_pixels,
            screen.height_in_pixels,
            0,
            1,
            screen.root_visual,
            2048,
            &mut value_list[0],
        );

        // Make fullscreen.
        xcb_change_property(
            connection,
            0,
            window,
            get_atom(connection, b"_NET_WM_STATE"),
            4,
            32,
            1,
            [get_atom(connection, b"_NET_WM_STATE_FULLSCREEN")].as_ptr() as *const _,
        );

        // Enable operating system to close the window.
        xcb_change_property(
            connection,
            0,
            window,
            get_atom(connection, b"WM_PROTOCOLS"),
            4,
            32,
            1,
            [get_atom(connection, b"WM_DELETE_WINDOW")].as_ptr() as *const _,
        );

        // Set the window name.
        xcb_change_property(
            connection,
            0,
            window,
            /* XCB_ATOM_WM_NAME */ 39,
            /* XCB_ATOM_STRING */ 31,
            8,
            11,
            b"Siyo Engine".as_ptr() as *const _ as *const c_void,
        );

        // Show The Window.
        xcb_map_window(connection, window);
        xcb_flush(connection);
    }

    // Return the window.
    window
}

fn get_atom(connection: *mut c_void, name: &[u8]) -> u32 {
    let atom = unsafe { xcb_intern_atom(connection, 0, name.len() as u16, &name[0]) };
    let reply = unsafe { xcb_intern_atom_reply(connection, atom, null_mut()) };
    let atom = unsafe {
        extern "C" {
            fn free(this: *mut XcbInternAtomReply) -> ();
        }
        let r_atom = (*reply).atom;
        free(reply);
        r_atom
    };
    atom
}

fn xkb_keyboard() -> (*mut c_void, *mut c_void, *mut c_void, *mut c_void) {
    use std::process::Command;

    let locale = std::ffi::CString::new(match std::env::var("LC_ALL") {
        Ok(val) => val,
        Err(_) => match std::env::var("LC_CTYPE") {
            Ok(val) => val,
            Err(_) => match std::env::var("LANG") {
                Ok(val) => val,
                Err(_) => "C".to_string(),
            },
        },
    })
    .unwrap();

    Command::new("xkbcomp")
        .arg("-xkb")
        .arg(&std::env::var("DISPLAY").unwrap())
        .arg("/tmp/xkbmap")
        .output()
        .expect("failed to execute process");

    let string =
        std::ffi::CString::new(std::fs::read_to_string("/tmp/xkbmap").expect("oops")).unwrap();

    let context = unsafe { xkb_context_new(ContextFlags::NoFlags) };
    let keymap =
        unsafe { xkb_keymap_new_from_string(context, string.as_ptr(), 1, CompileFlags::NoFlags) };
    let state = unsafe { xkb_state_new(keymap) };
    let compose_table = unsafe { xkb_compose_table_new_from_locale(context, locale.as_ptr(), 0) };
    let compose = unsafe { xkb_compose_state_new(compose_table, 0) };

    (state, keymap, context, compose)
}

/*// Convert width, height and xy to fixed point 32-bit.
fn cursor_convert(wh: (u16, u16), xy: (i16, i16)) -> (u32, u32) {
    let mut x = (xy.0 as u64) << 32;
    let mut y = (xy.0 as u64) << 32;
    x /= wh.0 as u64;
    y /= wh.1 as u64;
    (x as u32, y as u32)
}*/

fn xcb_poll_for_event2(
    connection: *mut c_void,
    state: *mut c_void,
    state2: *mut c_void,
    plotter: &mut footile::Plotter,
    raster: &mut footile::RasterB<footile::Rgba8>,
    rtn: &mut Internal,
    mode: &mut InputMode,
    input_sender: &std::sync::mpsc::Sender<InputMsg>,
) -> bool {
    extern "C" {
        fn free(event: *mut XcbGenericEvent) -> ();
    }

    let event = unsafe { xcb_poll_for_event(connection) };
    let event = if event.is_null() {
        return false;
    } else {
        unsafe {
            let r_event = (*event).clone();
            free(event);
            r_event
        }
    };

    let response_type = event.response_type;
    let detail = event.detail as u32;
    let event_xy = (event.event_x, event.event_y);

    // Handle keyboard input in text mode.
    if *mode == InputMode::Text {
        match response_type {
            2 => {
                unsafe {
                    let detail = xkb_state_key_get_one_sym(state, detail);

                    xkb_compose_state_feed(state2, detail);
                }

                match detail {
                    // Enter: Keyboard & NumPad
                    36 | 104 => input_sender.send(InputMsg::Text('\n')).unwrap(),
                    // Left & Right Shift, Alt Gr & NumLock & Esc & Caps Lock
                    50 | 62 | 108 | 77 | 9 | 66 => {
                        xkb_state_update_key2(state, detail, true);
                    }
                    // Everything else
                    _ => input_sender.send(InputMsg::Text(xkb_state_key_get_utf82(state, state2, detail))).unwrap(),
                }
            }
            3 => {
                xkb_state_update_key2(state, detail, false);
            }
            _ => { /*no text input*/ },
        }
    }

    match response_type {
        /*KEY_DOWN*/
        2 => match detail {
            // TODO
            /*ESCAPE*/
            9 => {
                // TODO: Require menu to exit.
                *rtn = Internal::Quit;
            }
            /*            /*W*/ 25 => adi::pc::wasd_set(adi::pc::WI),
            /*A*/ 38 => adi::pc::wasd_set(adi::pc::AJ),
            /*S*/ 39 => adi::pc::wasd_set(adi::pc::SK),
            /*D*/ 40 => adi::pc::wasd_set(adi::pc::DL),
            /*I*/ 31 => adi::pc::ijkl_set(adi::pc::WI),
            /*J*/ 44 => adi::pc::ijkl_set(adi::pc::AJ),
            /*K*/ 45 => adi::pc::ijkl_set(adi::pc::SK),
            /*L*/ 46 => adi::pc::ijkl_set(adi::pc::DL),*/
            _ => {}
        },
        /*KEY_UP*/
        3 => match detail {
            // TODO:
            /*            /*ESCAPE*/ 9 => adi::key_release(0, adi::Key::Back),
            /*W*/ 25 => adi::pc::wasd_unset(adi::pc::WI),
            /*A*/ 38 => adi::pc::wasd_unset(adi::pc::AJ),
            /*S*/ 39 => adi::pc::wasd_unset(adi::pc::SK),
            /*D*/ 40 => adi::pc::wasd_unset(adi::pc::DL),
            /*I*/ 31 => adi::pc::ijkl_unset(adi::pc::WI),
            /*J*/ 44 => adi::pc::ijkl_unset(adi::pc::AJ),
            /*K*/ 45 => adi::pc::ijkl_unset(adi::pc::SK),
            /*L*/ 46 => adi::pc::ijkl_unset(adi::pc::DL),*/
            _ => {}
        },
        /*BUTTON_DOWN*/
        4 => match detail {
            // TODO
            /*/*Left Click*/ 1 => { // Key::Press
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_press(0, adi::Key::Press);
            }
            /*Middle Click*/ 2 => { // Key::Cmd + Key::Press
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_press(0, adi::Key::L);
                adi::key_press(0, adi::Key::R);
            }
            /*Right Click*/ 3 => { // Key::Menu
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_press(0, adi::Key::Cmd);
                adi::key_press(0, adi::Key::Press);
            }
            /*Scroll Up*/ 4 => { // Left Throttle
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::set_lthrottle(0, 1.0);
            }
            /*Scroll Down*/ 5 => { // Right Throttle
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::set_rthrottle(0, 1.0);
            }*/
            _ => {} // Ignore all unknown clicks.
        },
        /*BUTTON_UP*/
        5 => match detail {
            // TODO
            /* /*Left Click*/ 1 => {
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_release(0, adi::Key::Press);
            }
            /*Middle Click*/ 2 => {
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_release(0, adi::Key::L);
                adi::key_release(0, adi::Key::R);
            }
            /*Right Click*/ 3 => {
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::key_release(0, adi::Key::Cmd);
                adi::key_release(0, adi::Key::Press);
            } // queue.right_button_release(*wh, event_xy),
            /*Scroll Up*/ 4 => {
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::set_lthrottle(0, 0.0);
            }
            /*Scroll Down*/ 5 => {
                Hid::cursor_set(cursor_convert(*wh, event_xy));
                adi::set_rthrottle(0, 0.0);
            }*/
            _ => {} // Ignore all unknown clicks.
        },
        /*CURSOR_MOVE*/ 6 => {} /*Hid::cursor_set(cursor_convert(*wh, event_xy))*/,
        /*GAIN_FOCUS/RESUME: TODO?*/ 9 => {}
        /*LOSE_FOCUS/PAUSE: TODO?*/ 10 => {}
        /*WINDOW_RESIZE*/ 22 => {
            *plotter = footile::Plotter::new(event.root_x as u32, event.root_y as u32);
            *raster = footile::RasterB::new(event.root_x as u32, event.root_y as u32);
            *rtn = Internal::Resize;
        }
        /*WINDOW_SELECT*/ 31 => println!("!SELECT!"),
        /*WINDOW_CLOSE*/
        161 => {
            // TODO: Require menu to exit.
            *rtn = Internal::Quit;
        }
        _ => {} // ignore all other messages
    }

    true
}

fn xkb_state_update_key2(state: *mut c_void, keycode: u32, dn: bool) {
    unsafe {
        xkb_state_update_key(
            state,
            keycode,
            if dn {
                KeyDirection::Down
            } else {
                KeyDirection::Up
            },
        );
    }
}

fn xkb_state_key_get_utf82(state: *mut c_void, state2: *mut c_void, key: u32) -> char {
    let status = unsafe { xkb_compose_state_get_status(state2) };

    match status {
        XkbComposeStatus::Composing => {
            return '\0';
        }
        XkbComposeStatus::Composed => {
            let size = unsafe {
                xkb_compose_state_get_utf8(state2, ::std::ptr::null_mut(), 0) as usize + 1
            };
            let mut utf8 = Vec::new();

            utf8.resize(size, b'\0'); // Size + 1 to include NULL byte from XKB.

            let buffer = utf8.as_mut_ptr();

            unsafe {
                xkb_compose_state_get_utf8(state2, buffer, size);
            }

            utf8.pop();

            return ::std::string::String::from_utf8(utf8)
                .unwrap()
                .chars()
                .next()
                .unwrap_or('\0');
        }
        _ => {}
    }

    let size =
        unsafe { xkb_state_key_get_utf8(state, key, ::std::ptr::null_mut(), 0) as usize + 1 };
    let mut utf8 = Vec::new();

    utf8.resize(size, b'\0'); // Size + 1 to include NULL byte from XKB.

    let buffer = utf8.as_mut_ptr();

    unsafe {
        xkb_state_key_get_utf8(state, key, buffer, size);
    }

    utf8.pop();

    ::std::string::String::from_utf8(utf8)
        .unwrap()
        .chars()
        .next()
        .unwrap_or('\0')
}

/*// Keycode translator
fn key(physical_key: u8) -> Option<u8> {
    Some(match physical_key {
        49 => keyboard::EXT_BACKTICK,
        86 => keyboard::EXT_PLUS,
        63 => keyboard::EXT_ASTERISK,
        61 | 106 => keyboard::SLASH,
        36 | 104 => keyboard::ENTER,
        10 | 87 => keyboard::NUM1,
        11 | 88 => keyboard::NUM2,
        12 | 89 => keyboard::NUM3,
        13 | 83 => keyboard::NUM4,
        14 | 84 => keyboard::NUM5,
        15 | 85 => keyboard::NUM6,
        16 | 79 => keyboard::NUM7,
        17 | 80 => keyboard::NUM8,
        18 | 81 => keyboard::NUM9,
        19 | 90 => keyboard::NUM0,
        60 | 91 => keyboard::PERIOD,
        20 | 82 => keyboard::MINUS,
        21 => keyboard::EQUAL_SIGN,
        22 => keyboard::BACKSPACE,
        23 => keyboard::TAB,
        38 => keyboard::A,
        56 => keyboard::B,
        54 => keyboard::C,
        40 => keyboard::D,
        26 => keyboard::E,
        41 => keyboard::F,
        42 => keyboard::G,
        43 => keyboard::H,
        31 => keyboard::I,
        44 => keyboard::J,
        45 => keyboard::K,
        46 => keyboard::L,
        58 => keyboard::M,
        57 => keyboard::N,
        32 => keyboard::O,
        33 => keyboard::P,
        24 => keyboard::Q,
        27 => keyboard::R,
        39 => keyboard::S,
        28 => keyboard::T,
        30 => keyboard::U,
        55 => keyboard::V,
        25 => keyboard::W,
        53 => keyboard::X,
        29 => keyboard::Y,
        52 => keyboard::Z,
        34 => keyboard::BRACKET_OPEN,
        35 => keyboard::BRACKET_CLOSE,
        37 => keyboard::LCTRL,
        105 => keyboard::RCTRL,
        50 => keyboard::LSHIFT,
        62 => keyboard::RSHIFT,
        64 => keyboard::ALT,
        108 => keyboard::EXT_ALT_GR,
        47 => keyboard::SEMICOLON,
        48 => keyboard::APOSTROPHE,
        51 => keyboard::BACKSLASH,
        59 => keyboard::COMMA,
        65 => keyboard::SPACE,
        77 => keyboard::EXT_NUM_LOCK,
        110 => keyboard::EXT_HOME,
        115 => keyboard::EXT_END,
        112 => keyboard::EXT_PAGE_UP,
        117 => keyboard::EXT_PAGE_DOWN,
        118 => keyboard::EXT_INSERT,
        119 => keyboard::EXT_DELETE,
        111 => keyboard::UP,
        113 => keyboard::LEFT,
        114 => keyboard::RIGHT,
        116 => keyboard::DOWN,
        _ => return None,
    })
}*/

// TODO: Don't allow unused.

// GL Types
#[allow(unused)]
pub type GLuint = u32;
#[allow(unused)]
pub type GLint = i32;
#[allow(unused)]
pub type GLenum = u32;
#[allow(unused)]
pub type GLboolean = u8;
#[allow(unused)]
pub type GLsizei = i32;
#[allow(unused)]
pub type GLchar = i8;
#[allow(unused)]
pub type GLbitfield = u32;
#[allow(unused)]
pub type GLsizeiptr = isize;
#[allow(unused)]
pub type GLfloat = f32;
#[allow(unused)]
pub type GLubyte = u8;

// X11 & Android
#[allow(unused)]
pub type EGLSurface = *mut c_void;
#[allow(unused)]
pub type EGLNativeWindowType = *mut c_void;
#[allow(unused)]
pub type EGLNativeDisplayType = *mut c_void;
#[allow(unused)]
pub type EGLDisplay = *mut c_void;
#[allow(unused)]
pub type EGLint = i32;
#[allow(unused)]
pub type EGLBoolean = u32;
#[allow(unused)]
pub type EGLConfig = *mut c_void;
#[allow(unused)]
pub type EGLContext = *mut c_void;
#[allow(unused)]
pub type EGLenum = u32;

#[allow(unused)]
pub const GL_FLOAT: u32 = 0x1406;
#[allow(unused)]
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
#[allow(unused)]
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
#[allow(unused)]
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
#[allow(unused)]
pub const GL_NEAREST: i32 = 0x2600;
#[allow(unused)]
pub const GL_LINEAR: i32 = 0x2601;
#[allow(unused)]
pub const GL_LINEAR_MIPMAP_LINEAR: i32 = 0x2703;
#[allow(unused)]
pub const GL_NEAREST_MIPMAP_NEAREST: i32 = 0x2700;
#[allow(unused)]
pub const GL_NEAREST_MIPMAP_LINEAR: i32 = 0x2702;
#[allow(unused)]
pub const GL_RGBA: u32 = 0x1908;
#[allow(unused)]
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;

#[allow(unused)]
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
#[allow(unused)]
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
#[allow(unused)]
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;

#[allow(unused)]
pub const EGL_BUFFER_SIZE: i32 = 0x3020;
#[allow(unused)]
pub const EGL_ALPHA_SIZE: i32 = 0x3021;
#[allow(unused)]
pub const EGL_BLUE_SIZE: i32 = 0x3022;
#[allow(unused)]
pub const EGL_GREEN_SIZE: i32 = 0x3023;
#[allow(unused)]
pub const EGL_RED_SIZE: i32 = 0x3024;
#[allow(unused)]
pub const EGL_DEPTH_SIZE: i32 = 0x3025;
#[allow(unused)]
pub const EGL_STENCIL_SIZE: i32 = 0x3026;
#[allow(unused)]
pub const EGL_CONFIG_CAVEAT: i32 = 0x3027;
#[allow(unused)]
pub const EGL_CONFIG_ID: i32 = 0x3028;
#[allow(unused)]
pub const EGL_LEVEL: i32 = 0x3029;
#[allow(unused)]
pub const EGL_MAX_PBUFFER_HEIGHT: i32 = 0x302A;
#[allow(unused)]
pub const EGL_MAX_PBUFFER_PIXELS: i32 = 0x302B;
#[allow(unused)]
pub const EGL_MAX_PBUFFER_WIDTH: i32 = 0x302C;
#[allow(unused)]
pub const EGL_NATIVE_RENDERABLE: i32 = 0x302D;
#[allow(unused)]
pub const EGL_NATIVE_VISUAL_ID: i32 = 0x302E;
#[allow(unused)]
pub const EGL_NATIVE_VISUAL_TYPE: i32 = 0x302F;
#[allow(unused)]
pub const EGL_SAMPLES: i32 = 0x3031;
#[allow(unused)]
pub const EGL_SAMPLE_BUFFERS: i32 = 0x3032;
#[allow(unused)]
pub const EGL_SURFACE_TYPE: i32 = 0x3033;
#[allow(unused)]
pub const EGL_TRANSPARENT_TYPE: i32 = 0x3034;
#[allow(unused)]
pub const EGL_TRANSPARENT_BLUE_VALUE: i32 = 0x3035;
#[allow(unused)]
pub const EGL_TRANSPARENT_GREEN_VALUE: i32 = 0x3036;
#[allow(unused)]
pub const EGL_TRANSPARENT_RED_VALUE: i32 = 0x3037;
#[allow(unused)]
pub const EGL_NONE: i32 = 0x3038;
#[allow(unused)]
pub const EGL_BIND_TO_TEXTURE_RGB: i32 = 0x3039;
#[allow(unused)]
pub const EGL_BIND_TO_TEXTURE_RGBA: i32 = 0x303A;
#[allow(unused)]
pub const EGL_MIN_SWAP_INTERVAL: i32 = 0x303B;
#[allow(unused)]
pub const EGL_MAX_SWAP_INTERVAL: i32 = 0x303C;

#[allow(unused)]
pub const EGL_DONT_CARE: i32 = -1;
#[allow(unused)]
pub const EGL_SLOW_CONFIG: i32 = 0x3050;
#[allow(unused)]
pub const EGL_NON_CONFORMANT_CONFIG: i32 = 0x3051;
#[allow(unused)]
pub const EGL_TRANSPARENT_RGB: i32 = 0x3052;
#[allow(unused)]
pub const EGL_NO_TEXTURE: i32 = 0x305C;
#[allow(unused)]
pub const EGL_TEXTURE_RGB: i32 = 0x305D;
#[allow(unused)]
pub const EGL_TEXTURE_RGBA: i32 = 0x305E;
#[allow(unused)]
pub const EGL_TEXTURE_2D: i32 = 0x305F;

#[allow(unused)]
pub const EGL_PBUFFER_BIT: i32 = 0x01;
#[allow(unused)]
pub const EGL_PIXMAP_BIT: i32 = 0x02;
#[allow(unused)]
pub const EGL_WINDOW_BIT: i32 = 0x04;

#[allow(unused)]
pub const EGL_VENDOR: i32 = 0x3053;
#[allow(unused)]
pub const EGL_VERSION: i32 = 0x3054;
#[allow(unused)]
pub const EGL_EXTENSIONS: i32 = 0x3055;

#[allow(unused)]
pub const EGL_HEIGHT: i32 = 0x3056;
#[allow(unused)]
pub const EGL_WIDTH: i32 = 0x3057;
#[allow(unused)]
pub const EGL_LARGEST_PBUFFER: i32 = 0x3058;
#[allow(unused)]
pub const EGL_TEXTURE_FORMAT: i32 = 0x3080;
#[allow(unused)]
pub const EGL_TEXTURE_TARGET: i32 = 0x3081;
#[allow(unused)]
pub const EGL_MIPMAP_TEXTURE: i32 = 0x3082;
#[allow(unused)]
pub const EGL_MIPMAP_LEVEL: i32 = 0x3083;

#[allow(unused)]
pub const EGL_BACK_BUFFER: i32 = 0x3084;

#[allow(unused)]
pub const EGL_DRAW: i32 = 0x3059;
#[allow(unused)]
pub const EGL_READ: i32 = 0x305A;

#[allow(unused)]
pub const EGL_CORE_NATIVE_ENGINE: i32 = 0x305B;

#[allow(unused)]
pub const EGL_RENDERABLE_TYPE: i32 = 0x3040;
#[allow(unused)]
pub const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
#[allow(unused)]
pub const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;

#[allow(unused)]
pub const EGL_OPENGL_ES_API: u32 = 0x30A0;

#[link(name = "EGL")]
extern "C" {
    fn eglGetDisplay(a: EGLNativeDisplayType) -> EGLDisplay;
    fn eglInitialize(a: EGLDisplay, b: *mut EGLint, c: *mut EGLint) -> EGLBoolean;
    fn eglChooseConfig(
        a: EGLDisplay,
        b: *const EGLint,
        c: *mut EGLConfig,
        d: EGLint,
        e: *mut EGLint,
    ) -> EGLBoolean;
    fn eglCreateContext(a: EGLDisplay, b: EGLConfig, c: EGLContext, d: *const EGLint)
        -> EGLContext;
    fn eglGetConfigAttrib(a: EGLDisplay, b: EGLConfig, c: EGLint, d: *mut EGLint) -> EGLBoolean;
    fn eglBindAPI(a: EGLenum) -> EGLBoolean;
    fn eglSwapBuffers(a: EGLDisplay, b: EGLSurface) -> EGLBoolean;
    fn eglGetProcAddress(a: *const i8) -> *mut c_void;
    fn eglCreateWindowSurface(
        a: EGLDisplay,
        b: EGLConfig,
        c: EGLNativeWindowType,
        d: *const EGLint,
    ) -> EGLSurface;
    fn eglMakeCurrent(a: EGLDisplay, b: EGLSurface, c: EGLSurface, d: EGLContext) -> EGLBoolean;
    fn eglSwapInterval(a: EGLDisplay, b: EGLint) -> EGLBoolean;
}

pub struct Display {
    display: *mut c_void,
    surface: Option<std::ptr::NonNull<c_void>>,
    config: *mut c_void,
    context: *mut c_void,
}

impl Display {
    // Swap surface with screen buffer.
    pub fn swap(&self) {
        if unsafe { eglSwapBuffers(self.display, self.surface.unwrap().as_ptr()) } == 0 {
            panic!("Swapping Failed");
        }
    }
}

/// Initialize the opengl (connect to the display)
pub fn init() -> (Display, i32) {
    let display = unsafe { eglGetDisplay(std::ptr::null_mut()) };
    if display.is_null() {
        panic!("EGL: Couldn't load display.");
    }

    if unsafe { eglInitialize(display, std::ptr::null_mut(), std::ptr::null_mut()) } == 0 {
        panic!("Couldn't initialize EGL");
    }

    // Config
    let mut config = std::ptr::null_mut();
    let mut nconfigs = unsafe { std::mem::uninitialized() };

    if unsafe {
        eglChooseConfig(
            display,
            [
                EGL_RED_SIZE,
                8,
                EGL_GREEN_SIZE,
                8,
                EGL_BLUE_SIZE,
                8,
                EGL_DEPTH_SIZE,
                24,
                EGL_NONE,
            ]
            .as_ptr(),
            &mut config,
            1,
            &mut nconfigs,
        )
    } == 0
    {
        panic!("Couldn't choose the config");
    }

    if nconfigs == 0 {
        panic!("No configs!");
    }

    if unsafe { eglBindAPI(EGL_OPENGL_ES_API) } == 0 {
        panic!("Couldn't bind OpenGLES");
    }

    // Create an EGL rendering context.
    let context = unsafe {
        eglCreateContext(
            display,
            config,
            std::ptr::null_mut(),
            [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE].as_ptr(),
        )
    };

    if context.is_null() {
        panic!("Couldn't create EGL rendering context.");
    }

    let surface = None;

    // Get visual id
    let mut visual_id = unsafe { std::mem::uninitialized() };
    if unsafe { eglGetConfigAttrib(display, config, EGL_NATIVE_VISUAL_ID, &mut visual_id) } == 0 {
        panic!("couldn't get visual id");
    }

    (
        Display {
            display,
            surface,
            config,
            context,
        },
        visual_id,
    )
}

/// Initialize the opengl (connect to the display) STEP 2
pub fn init2(display: &mut Display, window: EGLNativeWindowType) {
    // Create surface
    let surface = unsafe {
        eglCreateWindowSurface(display.display, display.config, window, std::ptr::null())
    };

    if surface.is_null() {
        panic!("Couldn't create EGL surface.");
    }

    // Connect context to surface
    if unsafe { eglMakeCurrent(display.display, surface, surface, display.context) } == 0 {
        panic!("Couldn't make current");
    }

    // Synchronize buffer swaps to monitor refresh rate.
    unsafe { eglSwapInterval(display.display, 1) };

    // Guaranteed to be `Some` because of conditional panic above.
    display.surface = std::ptr::NonNull::new(surface);
}

/// The OpenGL context.
pub(crate) struct OpenGLContext {
//    clear: unsafe extern "system" fn(GLbitfield) -> (),
//    clear_color: unsafe extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> (),
    disable: unsafe extern "system" fn(GLenum) -> (),
    enable: unsafe extern "system" fn(GLenum) -> (),
    front_face: unsafe extern "system" fn(GLenum) -> (),
    blend_func_separate: unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> (),
    create_shader: unsafe extern "system" fn(GLenum) -> GLuint,
    shader_source:
        unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> (),
    compile_shader: unsafe extern "system" fn(GLuint) -> (),
    create_program: unsafe extern "system" fn() -> GLuint,
    attach_shader: unsafe extern "system" fn(GLuint, GLuint) -> (),
    link_program: unsafe extern "system" fn(GLuint) -> (),
//    uniform: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint,
    gen_buffers: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
    bind_buffer: unsafe extern "system" fn(GLenum, GLuint) -> (),
    buffer_data: unsafe extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) -> (),
    vdata: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint,
    draw_arrays: unsafe extern "system" fn(GLenum, GLint, GLsizei) -> (),
    use_program: unsafe extern "system" fn(GLuint) -> (),
//    uniform_mat4: unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
//    uniform_vec1: unsafe extern "system" fn(GLint, GLfloat) -> (),
//    uniform_vec4: unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
    bind_texture: unsafe extern "system" fn(GLenum, GLuint) -> (),
    bind_framebuffer: unsafe extern "system" fn(GLenum, GLuint) -> (),
    attach_framebuffer: unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> (),
    vertex_attrib:
        unsafe extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) -> (),
    gen_textures: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
    gen_framebuffers: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
    tex_params: unsafe extern "system" fn(GLenum, GLenum, GLint) -> (),
    tex_image: unsafe extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const c_void,
    ) -> (),
    enable_vdata: unsafe extern "system" fn(GLuint) -> (),
    viewport: unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> (),
    detach_shader: unsafe extern "system" fn(GLuint, GLuint) -> (),
    delete_program: unsafe extern "system" fn(GLuint) -> (),
    delete_buffer: unsafe extern "system" fn(GLsizei, *const GLuint) -> (),
    delete_texture: unsafe extern "system" fn(GLsizei, *const GLuint) -> (),
    delete_fb: unsafe extern "system" fn(GLsizei, *mut GLuint) -> (),
}

fn load_check(name: &[u8], fn_ptr: *const c_void) {
    if fn_ptr.is_null() {
        panic!(
            "couldn't load function \"{}\"!",
            std::str::from_utf8(name).unwrap()
        );
    }
}

// Load an OpenGL 3 / OpenGLES 2 function.
pub fn load<T>(name: &[u8]) -> T {
    let fn_ptr: *const c_void = unsafe { eglGetProcAddress(name as *const _ as *const i8) };
    load_check(name, fn_ptr);
    unsafe { std::mem::transmute_copy::<*const c_void, T>(&fn_ptr) }
}

impl OpenGLContext {
    pub fn new() -> OpenGLContext {
        OpenGLContext {
            // FFI OpenGL Functions.
//            clear: load(b"glClear\0"),
//            clear_color: load(b"glClearColor\0"),
            disable: load(b"glDisable\0"),
            enable: load(b"glEnable\0"),
            front_face: load(b"glFrontFace\0"),
            blend_func_separate: load(b"glBlendFuncSeparate\0"),
            create_shader: load(b"glCreateShader\0"),
            shader_source: load(b"glShaderSource\0"),
            compile_shader: load(b"glCompileShader\0"),
            create_program: load(b"glCreateProgram\0"),
            attach_shader: load(b"glAttachShader\0"),
            link_program: load(b"glLinkProgram\0"),
//            uniform: load(b"glGetUniformLocation\0"),
            gen_buffers: load(b"glGenBuffers\0"),
            bind_buffer: load(b"glBindBuffer\0"),
            buffer_data: load(b"glBufferData\0"),
            vdata: load(b"glGetAttribLocation\0"),
            draw_arrays: load(b"glDrawArrays\0"),
            use_program: load(b"glUseProgram\0"),
//            uniform_mat4: load(b"glUniformMatrix4fv\0"),
//            uniform_vec1: load(b"glUniform1f\0"),
//            uniform_vec4: load(b"glUniform4f\0"),
            bind_texture: load(b"glBindTexture\0"),
            bind_framebuffer: load(b"glBindFramebuffer\0"),
            attach_framebuffer: load(b"glFramebufferTexture2D\0"),
            vertex_attrib: load(b"glVertexAttribPointer\0"),
            gen_textures: load(b"glGenTextures\0"),
            gen_framebuffers: load(b"glGenFramebuffers\0"),
            tex_params: load(b"glTexParameteri\0"),
            tex_image: load(b"glTexImage2D\0"),
            enable_vdata: load(b"glEnableVertexAttribArray\0"),
            viewport: load(b"glViewport\0"),
            detach_shader: load(b"glDetachShader\0"),
            delete_program: load(b"glDeleteProgram\0"),
            delete_buffer: load(b"glDeleteBuffers\0"),
            delete_texture: load(b"glDeleteTextures\0"),
            delete_fb: load(b"glDeleteFramebuffers\0"),
        }
    }
}

fn main() {
    let (mut display, id) = init();
    let mut window = Window::new(Some(id));
    init2(&mut display, unsafe {
        std::mem::transmute(window.window as usize)
    });
    let opengl = OpenGLContext::new();

    // Basic Rules needed to render.
    unsafe {
        // Set the settings.
        (opengl.disable)(/* Dither */ 0x0BD0u32);
        (opengl.disable)(/* Blend */ 0x0BE2u32);
        (opengl.enable)(/* CullFace */ 0x0B44u32);

        // Alpha Blending.
        (opengl.blend_func_separate)(
             /* GL_SRC_ALPHA */ 0x0302u32,
             /* GL_ONE_MINUS_SRC_ALPHA*/ 0x0303u32,
             /* GL_SRC_ALPHA */ 0x0302u32,
             /* GL_DST_ALPHA */ 0x0304u32,
        );

        // Set the viewport.
        (opengl.viewport)(0, 0, window.plotter.width() as GLsizei, window.plotter.height() as GLsizei);
    }

    // Build Shader Program
    const SHADER_TEX_VERT: &'static [u8] = b"\
        #version 100\n\
        precision mediump float;\n\
        attribute vec2 position;\n\
        attribute vec2 texpos;\n\
        varying vec2 texcoord;\n\
        void main() {\n\
	        gl_Position = vec4(position.xy, 0.0, 1.0);\n\
	        texcoord = texpos;\n\
        }";
    const SHADER_TEX_FRAG: &'static [u8] = b"\
        #version 100\n\
        precision mediump float;\n\
        uniform sampler2D texture;\n\
        varying vec2 texcoord;\n\
        void main() {\n\
	        gl_FragColor = texture2D(texture, texcoord);\n\
        }";
    let program = unsafe {
        // Compile Vertex Shaders
        let v_shader = (opengl.create_shader)(0x8B31 /*vertex*/);
        (opengl.shader_source)(
            v_shader,
            1,
            [SHADER_TEX_VERT.as_ptr() as *const _].as_ptr(),
            [SHADER_TEX_VERT.len() as i32].as_ptr(),
        );
        (opengl.compile_shader)(v_shader);
        // Compile Fragment Shader
        let f_shader = (opengl.create_shader)(0x8B30 /*fragment*/);
        (opengl.shader_source)(
            f_shader,
            1,
            [SHADER_TEX_FRAG.as_ptr() as *const _].as_ptr(),
            [SHADER_TEX_FRAG.len() as i32].as_ptr(),
        );
        (opengl.compile_shader)(f_shader);
        // Link shaders together.
        let program = (opengl.create_program)();
        (opengl.attach_shader)(program, v_shader);
        (opengl.attach_shader)(program, f_shader);
        (opengl.link_program)(program);
        (opengl.detach_shader)(program, v_shader);
        (opengl.detach_shader)(program, f_shader);
        (opengl.use_program)(program);
        program
    };
    // Get Vertex Attributes
    let position = unsafe { (opengl.vdata)(program, b"position\0".as_ptr() as *const _) };
    let texpos = unsafe { (opengl.vdata)(program, b"texpos\0".as_ptr() as *const _) };
    // Enable Vertex Attributes
    unsafe {
        (opengl.enable_vdata)(position as u32);
        (opengl.enable_vdata)(texpos as u32);
    }
    // Set Texture
    let mut blit_texture = unsafe { std::mem::uninitialized() };
    let mut screen_texture = unsafe { std::mem::uninitialized() };
    let mut fb = unsafe { std::mem::uninitialized() };
    let mut screen_fb = unsafe { std::mem::uninitialized() };
    unsafe {
        (opengl.gen_framebuffers)(1, &mut fb);
        (opengl.gen_textures)(1, &mut blit_texture);
        (opengl.bind_texture)(GL_TEXTURE_2D, blit_texture);
        // TODO: is most optimal?
        (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        // Set Texture. TODO: Resize
        (opengl.tex_image)(
            GL_TEXTURE_2D,
            0,
            GL_RGBA as i32,
            window.plotter.width() as i32, // w
            window.plotter.height() as i32, // h
            0,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            window.pixels.as_ptr() as *const _,
        );
        (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, fb);
        (opengl.attach_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, /* GL_COLOR_ATTACHMENT0 */ 0x8CE0,
            GL_TEXTURE_2D, blit_texture, 0);

        (opengl.gen_framebuffers)(1, &mut screen_fb);
        (opengl.gen_textures)(1, &mut screen_texture);
        (opengl.bind_texture)(GL_TEXTURE_2D, screen_texture);
        // TODO: is most optimal?
        (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        // Set Texture. TODO: Resize
        (opengl.tex_image)(
            GL_TEXTURE_2D,
            0,
            GL_RGBA as i32,
            window.plotter.width() as i32, // w
            window.plotter.height() as i32, // h
            0,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            window.pixels.as_ptr() as *const _,
        );
        (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, screen_fb);
        (opengl.attach_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, /* GL_COLOR_ATTACHMENT0 */ 0x8CE0,
            GL_TEXTURE_2D, screen_texture, 0);
    }
    // Set Position
    type VertexList = [f32; 2 * 6];
    let position_buffer = unsafe {
        let position_c: VertexList = [
            -1.0, 1.0,
            -1.0, -1.0,
            1.0, -1.0,
            1.0, -1.0,
            1.0, 1.0,
            -1.0, 1.0,
        ];
        let mut buffers = [std::mem::uninitialized()];
        (opengl.gen_buffers)(1 /*1 buffer*/, buffers.as_mut_ptr());
        (opengl.bind_buffer)(GL_ARRAY_BUFFER, buffers[0]);
        (opengl.buffer_data)(
            GL_ARRAY_BUFFER,
            std::mem::size_of::<VertexList>() as isize,
            position_c.as_ptr() as *const _,
            GL_DYNAMIC_DRAW,
        );
        (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
        buffers[0]
    };
    let bitmap_buffer = unsafe {
        let position_c: VertexList = [
            -1.0, 1.0,
            -1.0, -1.0,
            1.0, -1.0,
            1.0, -1.0,
            1.0, 1.0,
            -1.0, 1.0,
        ];
        let mut buffers = [std::mem::uninitialized()];
        (opengl.gen_buffers)(1 /*1 buffer*/, buffers.as_mut_ptr());
        (opengl.bind_buffer)(GL_ARRAY_BUFFER, buffers[0]);
        (opengl.buffer_data)(
            GL_ARRAY_BUFFER,
            std::mem::size_of::<VertexList>() as isize,
            position_c.as_ptr() as *const _,
            GL_DYNAMIC_DRAW,
        );
        (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
        buffers[0]
    };
    // Set Texture Coordinates
    let texpos_buffer = unsafe {
        let texpos_c: VertexList = [
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
        ];
        let mut buffers = [std::mem::uninitialized()];
        (opengl.gen_buffers)(1 /*1 buffer*/, buffers.as_mut_ptr());
        (opengl.bind_buffer)(GL_ARRAY_BUFFER, buffers[0]);
        (opengl.buffer_data)(
            GL_ARRAY_BUFFER,
            std::mem::size_of::<VertexList>() as isize,
            texpos_c.as_ptr() as *const _,
            GL_DYNAMIC_DRAW,
        );
        (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
        buffers[0]
    };
    // Set Texture Coordinates
    let screen_texpos_buffer = unsafe {
        let texpos_c: VertexList = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];
        let mut buffers = [std::mem::uninitialized()];
        (opengl.gen_buffers)(1 /*1 buffer*/, buffers.as_mut_ptr());
        (opengl.bind_buffer)(GL_ARRAY_BUFFER, buffers[0]);
        (opengl.buffer_data)(
            GL_ARRAY_BUFFER,
            std::mem::size_of::<VertexList>() as isize,
            texpos_c.as_ptr() as *const _,
            GL_DYNAMIC_DRAW,
        );
        (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
        buffers[0]
    };

    println!("Library init complete! / Starting new thread....");
    // Init Multi-Threading Data Structures
    let (input_sender, input_receiver) = std::sync::mpsc::channel::<InputMsg>();
    let (output_sender, output_receiver) = std::sync::mpsc::channel::<(bool, Vec<OutputMsg>)>();
    let output_sender2 = output_sender.clone();
    std::thread::spawn(move || {
        user_thread(output_sender2, input_receiver);
    });
    // Init Render Data
    let mut path = vec![];
    let mut fill = [0x00, 0x00, 0x00, 0xFF];
    let mut stroke = [0x00, 0x00, 0x00, 0xFF];
    let mut matrix = {
        let width = window.plotter.width() as f32;
        perspective(/*std::f32::consts::PI * 0.5,*/ width, 1.0, 100.0)
    };
    let mut textures = vec![];
    let mut old_output_msg = Vec::new();
    println!("Entering Loop....");
    loop {
        // Render to screen
        unsafe { (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, 0) };
//        unsafe { (opengl.clear)(0x00000100 | 0x00004000) };
        unsafe {
            // Bind position_buffer to position
            (opengl.bind_buffer)(GL_ARRAY_BUFFER, position_buffer);
            (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
            // Bind texpos_buffer to texpos
            (opengl.bind_buffer)(GL_ARRAY_BUFFER, screen_texpos_buffer);
            (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
            (opengl.bind_texture)(GL_TEXTURE_2D, screen_texture);
            (opengl.draw_arrays)(
                /* Triangles */ 0x0004 as GLuint,
                /* Start */ 0 as GLint,
                /* End */ 6 as GLsizei,
            )
        }
        display.swap();
        // Input: Send Messages.
        match window.update(&input_sender) {
            Internal::None => { /* do nothing */ }
            Internal::Quit => { break }
            Internal::Resize => {
                // Set the viewport.
                unsafe {
                    (opengl.viewport)(0, 0, window.plotter.width() as GLsizei, window.plotter.height() as GLsizei);
                }
                // Update matrix
                matrix = {
                    let width = window.plotter.width() as f32;
                    perspective(/*std::f32::consts::PI * 0.5, */width, 1.0, 100.0)
                };
                window.pixels.resize(window.plotter.width() as usize * window.plotter.height() as usize * 4, 0);
                // Resend previous messages.
                output_sender.send((false, old_output_msg.clone())).unwrap();
            }
        }
        // Ouput: Receive messages.
        unsafe { (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, fb) };
        unsafe { (opengl.viewport)(0, 0, window.plotter.width() as GLsizei, window.plotter.height() as GLsizei) };

        if let Ok((save, output_msg)) = output_receiver.try_recv() {
            if save {
                old_output_msg = output_msg.clone();
            }
            for msg in output_msg.iter() {
                match msg.clone() {
                    OutputMsg::InputMode(_) => { /* TODO */ }
                    OutputMsg::Page(_) => { /* TODO */ }
                    OutputMsg::Menu(_) => { /* TODO */ }
                    OutputMsg::DrawBitmap(positions, bitmap_id) => {
                        let vectora = matrix * cgmath::Vector4::new(
                            positions[0][0],
                            positions[0][1],
                            positions[0][2],
                            1.0
                        );
                        let vectorb = matrix * cgmath::Vector4::new(
                            positions[1][0],
                            positions[1][1],
                            positions[1][2],
                            1.0
                        );
                        let vectorc = matrix * cgmath::Vector4::new(
                            positions[2][0],
                            positions[2][1],
                            positions[2][2],
                            1.0
                        );
                        let vectord = matrix * cgmath::Vector4::new(
                            positions[3][0],
                            positions[3][1],
                            positions[3][2],
                            1.0
                        );
                        let w = window.plotter.width() as f32;
                        let h = window.plotter.height() as f32;
                        let position_c: VertexList = [
                            vectora[0] / w * 2.0 - 1.0, vectora[1] / h * 2.0 - 1.0,
                            vectord[0] / w * 2.0 - 1.0, vectord[1] / h * 2.0 - 1.0,
                            vectorc[0] / w * 2.0 - 1.0, vectorc[1] / h * 2.0 - 1.0,
                            vectorc[0] / w * 2.0 - 1.0, vectorc[1] / h * 2.0 - 1.0,
                            vectorb[0] / w * 2.0 - 1.0, vectorb[1] / h * 2.0 - 1.0,
                            vectora[0] / w * 2.0 - 1.0, vectora[1] / h * 2.0 - 1.0,
                        ];
                        unsafe {
                            (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, fb);

                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, bitmap_buffer);
                            (opengl.buffer_data)(
                                GL_ARRAY_BUFFER,
                                std::mem::size_of::<VertexList>() as isize,
                                position_c.as_ptr() as *const _,
                                GL_DYNAMIC_DRAW,
                            );
                            (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, texpos_buffer);
                            (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            (opengl.bind_texture)(GL_TEXTURE_2D, textures[bitmap_id as usize]);
                            (opengl.front_face)(/* Clockwise */ 0x0900u32);
                            (opengl.draw_arrays)(
                                /* Triangles */ 0x0004 as GLuint,
                                /* Start */ 0 as GLint,
                                /* End */ 6 as GLsizei,
                            );
                            (opengl.front_face)(/* Clockwise */ 0x0901u32);

                            (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, screen_fb);

                            // Bind position_buffer to position
                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, position_buffer);
                            (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            // Bind texpos_buffer to texpos
                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, texpos_buffer);
                            (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            (opengl.bind_texture)(GL_TEXTURE_2D, blit_texture);
                            (opengl.draw_arrays)(
                                /* Triangles */ 0x0004 as GLuint,
                                /* Start */ 0 as GLint,
                                /* End */ 6 as GLsizei,
                            )
                        }
                    }
                    OutputMsg::Clear(color) => {
                        let mut modulo = 0;
                        for i in window.pixels.as_mut_slice() {
                            let index = modulo % 4;
                            *i = color[index];
                            modulo += 1;
                        }
                    }
                    OutputMsg::Fill(color) => {
                        fill = color;
                    }
                    OutputMsg::Stroke(color) => {
                        stroke = color;
                    }
                    OutputMsg::Width(w) => {
                        path.push(footile::PathOp::PenWidth(w));
                    }
                    OutputMsg::Matrix(m) => {
                        
                    }
                    OutputMsg::Move(x, y, z) => {
                        if path.is_empty() == false {
                            path.push(footile::PathOp::Close());
                        }
                        // w is 1 because position
                        let vector = matrix * cgmath::Vector4::new(x, y, z, 1.0);
                        path.push(footile::PathOp::Move(vector.x, vector.y));
                    }
                    OutputMsg::Line(x, y, z) => {
                        // w is 1 because position
                        let vector = matrix * cgmath::Vector4::new(x, y, z, 1.0);
                        path.push(footile::PathOp::Line(vector.x, vector.y));
                    }
                    OutputMsg::Quad((cx, cy, cz), x, y, z) => {
                        let cvector = matrix * cgmath::Vector4::new(cx, cy, cz, 1.0);
                        let vector = matrix * cgmath::Vector4::new(x, y, z, 1.0);
                        path.push(footile::PathOp::Quad(cvector.x, cvector.y, vector.x, vector.y));
                    }
                    OutputMsg::Cube((ax, ay, az), (bx, by, bz), x, y, z) => {
                        let avector = matrix * cgmath::Vector4::new(ax, ay, az, 1.0);
                        let bvector = matrix * cgmath::Vector4::new(bx, by, bz, 1.0);
                        let vector = matrix * cgmath::Vector4::new(x, y, z, 1.0);
                        path.push(footile::PathOp::Cubic(avector.x, avector.y, bvector.x, bvector.y, vector.x, vector.y));
                    }
                    OutputMsg::Close() => {
                        path.push(footile::PathOp::Close());
                        window.raster.over(window.plotter.fill(&path, footile::FillRule::NonZero), footile::Rgba8::new(fill[0], fill[1], fill[2], fill[3]), footile::Rgba8::as_slice_mut(window.pixels.as_mut_slice()));
                        path.clear();
                    }
                    OutputMsg::Sync() => {
                        unsafe {
                            (opengl.bind_framebuffer)(/* GL_FRAMEBUFFER */ 0x8D40, screen_fb);

                            // Bind position_buffer to position
                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, position_buffer);
                            (opengl.vertex_attrib)(position as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            // Bind texpos_buffer to texpos
                            (opengl.bind_buffer)(GL_ARRAY_BUFFER, texpos_buffer);
                            (opengl.vertex_attrib)(texpos as GLuint, 2, GL_FLOAT, 0, 0, std::ptr::null());
                            (opengl.bind_texture)(GL_TEXTURE_2D, blit_texture);
                            (opengl.tex_image)(
                                GL_TEXTURE_2D,
                                0,
                                GL_RGBA as i32,
                                window.plotter.width() as i32, // w
                                window.plotter.height() as i32, // h
                                0,
                                GL_RGBA,
                                GL_UNSIGNED_BYTE,
                                window.pixels.as_ptr() as *const _,
                            );
                            (opengl.draw_arrays)(
                                /* Triangles */ 0x0004 as GLuint,
                                /* Start */ 0 as GLint,
                                /* End */ 6 as GLsizei,
                            )
                        }
                    }
                    OutputMsg::Text(_) => {}
                    OutputMsg::LoadBitmap(id, width, pixels) => {
                        // Loading a texture must be consecutive or replacement.
                        if id as usize > textures.len() {
                            println!("Loaded in the wrong order!");
                            panic!();
                        } else if id as usize == textures.len() {
                            println!("LADOING");
                            let mut new_texture = unsafe { ::std::mem::uninitialized() };
                            unsafe {
                                (opengl.gen_textures)(1, &mut new_texture);
                                (opengl.bind_texture)(GL_TEXTURE_2D, new_texture);
                                // TODO: is most optimal?
                                (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
                                (opengl.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
                                // Set Texture. TODO: Resize
                                (opengl.tex_image)(
                                    GL_TEXTURE_2D,
                                    0,
                                    GL_RGBA as i32,
                                    width as i32, // w
                                    ((pixels.len() >> 2) as i32) / (width as i32), // h
                                    0,
                                    GL_RGBA,
                                    GL_UNSIGNED_BYTE,
                                    pixels.as_ptr() as *const _,
                                );
                            }
                            textures.push(new_texture);
                        } else {
                            println!("Load Bitmap");
                            unimplemented!();
                        }
                    }
                }
            }
        }
    }

    println!("Cleaning up....");
    unsafe {
        (opengl.delete_program)(program);
        (opengl.delete_texture)(1, &blit_texture);
        for texture in textures.iter() {
            (opengl.delete_texture)(1, texture);
        }
        (opengl.delete_fb)(1, &mut fb);
        (opengl.delete_buffer)(2, [position_buffer, texpos_buffer].as_ptr());
    }
}
