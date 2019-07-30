//! # Getting Started
//! For HID (Human Interface Device's), there must be separate modes for limited input (like on a phone).  This library takes care of the different input modes for you.
//!
//! ## Input Modes
//! * push - This input mode handles Touch and Click.  This is good for apps & phone games.  Only use hover for highlighting buttons.
//! * keys - This input mode handles Keyboard, Touch and Click.  This is good for when you need the user to type.  On touchscreens, this shows an on-screen keyboard.  So, when the user is done typing, you should go back to Tap mode.
//! * ctrl - This input mode handles Controllers.  This is good for video games & even possibly 3D modeling software.  If specific controls aren't available, GUI controls are created.
//! ```
//! // TODO
//! ```
//!
//! # Controller
//! There are many different kinds of controllers, and unfortunately they have different buttons.
//! To work around this, controllers are mapped to the imaginary controller pictured below.
//!
//! <img src="https://free.plopgrizzly.com/siyo/res/controller.png" width="642px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
//!
//! ## Combinations
//! To increase the number of controls, some combinations are automatically implemented.  The
//! combinations all involve the Up or Down button.  The Up and Down buttons can only get release
//! events, and must be pressed first for the combos.
//! * Up + LStick = RStick
//! * Down + Hi = Exec
//! * Down + Lo = Back

static mut HID: Hid = Hid {
    d: 0,
    i: 0,
    p: 0,
    x: 0,
    y: 0,
    k: 0,
    c: 0,
    m: 0,
    v: 0,
    h: 0,
    l: 0,
    r: 0,
};

const HID_DTHROTTLE: u8 = 0b_1000_0000;
const HID_DPUSH: u8 = 0b_0100_0000;
const HID_DKEYS: u8 = 0b_0010_0000;
const HID_DCTRL: u8 = 0b_0001_0000;
const HID_DCURRENT: u8 = 0b_0000_1000;
const HID_DMEMORY: u8 = 0b_0000_0100;
const HID_DNEXT: u8 = 0b_0000_0001;

/// Human Input Device.
///
/// There are 3 different modes of input: Gui, Text, Game and 3 different types of controllers.
/// | Controller  | Gui Mode           | Text Mode          | Game Mode            |
/// |-------------|--------------------|--------------------|----------------------|
/// | Touchscreen | Touchscreen        | On-Screen Keyboard | On-Screen Controller |
/// | Computer    | Mouse: Left Click  | Keyboard           | WASD/Arrow/Mouse/Key |
/// | Joystick    | LStick & Ok Button | Controller Typer   | Controller           |
/// A "Push" that is not on an on-screen keyboard, brings you back to either touchscreen or joystick
/// mode, depending on which one was the previous mode.  If there is no on-screen keyboard, the Push
/// event is registered, otherwise it only exits the on-screen keyboard.
///
/// * `Push(WidgetID, State)` - The pushing of a button widget.  Buttons can be selected by pushing,
/// or WASD/HJKL/ARROW KEYS/TAB/SPACE/DPAD/LSTICK/RSTICK and then pressing ENTER/OK to push it.
///
/// * `Push` - Left Click/WASD,HJKL+Enter or Touch or Ok Button.
/// * `Drag` - Scroll Wheel/Shift-Scroll Wheel or Drag or D-Pad/L/R/ZL/ZR.
/// * `Back` - Escape/On-Screen-Back-Button or Back Key or Start Button.
/// * `Menu` - Long Hold Escape/On-Screen-Menu-Button or Menu Key or Long Hold Start Button.
#[repr(C)]
pub struct Hid {
    // 256 Bits
    // 32 Bits
    d: u8, // Pressed Down (Bits; 0: Throttle, 1: Push, 2: Keys, 3: Ctrl,
    // 4: Current, 5: Memory, 6: Previous Mode == Joystick, 7: Next?).
    p: u8,  // Pressure (0-255).
    i: u16, // GUI Index.
    // 64 bits
    x: u32, // Cursor X (Fixed Point 0-1 / left to right).
    y: u32, // Cursor Y (Fixed Point 0-1 / top to bottom).
    // 32 bits
    k: u32, // Unicode character (\0 for no character pressed down).
    // 128 Bits
    c: u32, // Buttons Currently Pressed Down.
    m: u32, // Buttons Memory Pressed Down.
    v: u16, // Joystick (lstick) Vertical Control.
    h: u16, // Joystick (lstick) Horizontal Control.
    l: u16, // Throttle Control.
    r: u16, // Right Throttle.
}

/// State of an input.
pub enum State {
    /// Not pressed down.
    None,
    /// Just pressed down.
    Just,
    /// Just Lifted (Released).
    Lift,
    /// Held down.
    Held,
}

impl Hid {
    fn get() -> &'static mut Hid {
        unsafe { &mut HID }
    }

    /// Is the push mode on?
    pub fn mode_push() -> bool {
        let hid = Hid::get();
        hid.d & HID_DPUSH != 0
    }

    /// Is the keyboard mode on?
    pub fn mode_keys() -> bool {
        let hid = Hid::get();
        hid.d & HID_DKEYS != 0
    }

    /// Is the controller mode on?
    pub fn mode_ctrl() -> bool {
        let hid = Hid::get();
        hid.d & HID_DCTRL != 0
    }

    /// Returns state of push control.
    #[inline(always)]
    pub fn push() -> State {
        let hid = Hid::get();
        let current = hid.d & HID_DCURRENT != 0;
        let memory = hid.d & HID_DMEMORY != 0;

        match (memory, current) {
            (false, false) => State::None,
            (false, true) => State::Just,
            (true, false) => State::Lift,
            (true, true) => State::Held,
        }
    }

    /// Get the keyboard input.
    pub fn keys() -> char {
        let hid = Hid::get();
        // We know it's valid.
        unsafe { std::char::from_u32_unchecked(hid.k) }
    }

    /// Get the cursor x and y fixed-point position.
    pub fn cursor() -> (u32, u32) {
        let hid = Hid::get();
        (hid.x, hid.y)
    }

    pub(crate) fn cursor_set(xy: (u32, u32)) {
        let hid = Hid::get();
        hid.x = xy.0;
        hid.y = xy.1;
    }

    pub(crate) fn push_set(down: bool) {
        let hid = Hid::get();
        let current = hid.d & HID_DCURRENT != 0;

        // If current is true, set memory to true; if current is false, set memory to false.
        if current {
            hid.d |= HID_DMEMORY;
        } else {
            hid.d &= !HID_DMEMORY;
        }

        // If down is true, set current to true; if down if false, set current to false.
        if down {
            hid.d |= HID_DCURRENT;
        } else {
            hid.d &= !HID_DCURRENT;
        }
    }

    pub(crate) fn keys_set(chr: char) {
        let hid = Hid::get();
        hid.k = chr as u32; // Guarantees valid char is in u32.
    }
}

/*
/// Get the mouse / touch position./
pub fn push() {

}

///
pub fn keys() {

}

///
pub fn ctrl() {

}


// mod ffi;

// pub(crate) use self::ffi::NativeManager;

// 128 bits memory for simulating one unified HID.
static mut HID_STATE: Option<Vec<HidState>> = None;

/// Input button for `HidState`.
///
/// # Buttons
/// For compatibility, phone has a menubar ontop.  It is:
/// ```
/// ← (Back), = (Menu)
/// ```
/// ```
///
/// Hi =>
/// Lo =>
/// Do => Left (Main) Click / Y Button / CapsLk Key / Phone: Touch
/// Ok => Right (Alt) Click / A Button / Enter Key / Phone: GUI Button K
/// ```
///
/// # General Game Control Suggestions
/// ```
/// Hi => Jump/Fly
/// Lo => Run/Sprint
/// Do => Attack/Activate
/// Ok => Talk/Select/Activate
/// Exec (Down + Hi) => Use Item
/// Near => Crouch / Sneak
/// Rstick (Up + LStick) => Camera (POV)
///
/// Up => Pick Up Item
/// Down => Set Down Item
/// Left => Choose Item Up (Previous)
/// Right => Choose Item Down (Next)
/// Back (Down + Lo) => Back/Menu
/// Far => Throw Item / Control Projectile (Shoot Bullet/Arrow, Cast Fishing Rod)
/// Lstick => Move
/// ```
///
/// # Extra Buttons
/// ```
/// RStickPush => Lo Toggle (Lock)
/// LStickPush => Near Toggle (Lock)
/// ```
///
/// # Conversions and Combos
/// * ExtR(0..25) - Extra Buttons (Right) `0-9,~,_,+,{,},|,:,",<,>,?,*,Alt`
/// * ExtR(26) - "Hi" Button or Space Key
/// * ExtR(27) - "Lo" Button or Ctrl Key
/// * ExtR(28) - "Do" Button or CapsLk Key or Left Click or Touch
/// * ExtR(29) - "Ok" Button or Enter Key or Right Click or Middle Click or Touch "K"
/// * ExtR(30) - "Exec" Button or Z(R) Button or Select Button or Tab Key or `Down + Hi`
/// * ExtR(31) - "Near" Button or Throttle Near or Shift Key
/// * ExtL(0..25) - Extra Buttons (Left) `A-Z`
/// * ExtL(26) - "Up" Button/Key/D-Pad
/// * ExtL(27) - "Down" Button/Key/D-Pad
/// * ExtL(28) - "Left" Button/Key/D-Pad or Scroll Up
/// * ExtL(29) - "Right" Button/Key/D-Pad or Scroll Down
/// * ExtL(30) - "Back" Button or ZL Button or Start Button or Escape Key or `Down + Lo`
/// * ExtL(31) - "Far" Button or Throttle Far or Backspace
#[derive(Copy, Clone, Debug)]
pub enum Key {
    /// <img src="https://free.plopgrizzly.com/siyo/res/Hi.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Hi.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Hi,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Lo.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Lo.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Lo,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Do.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Do.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Do,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Ok.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Ok.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Ok,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Exec.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Exec.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Exec,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Near.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Near.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Near,
    /// <img src="https://free.plopgrizzly.com/siyo/res/RStick.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/RStick.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    ExtR(u16),

    /// <img src="https://free.plopgrizzly.com/siyo/res/Up.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Up.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Up,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Down.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Down.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Down,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Left.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Left.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Left,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Right.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Right.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Right,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Back.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Back.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Back,
    /// <img src="https://free.plopgrizzly.com/siyo/res/Far.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/Far.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    Far,
    /// <img src="https://free.plopgrizzly.com/siyo/res/LStick.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    /// <img src="https://free.plopgrizzly.com/siyo/res/ps/LStick.png" width="32px" style="image-rendering:optimizeSpeed;image-rendering:-moz-crisp-edges;image-rendering:-o-crisp-edges;image-rendering:-webkit-optimize-contrast;image-rendering: optimize-contrast;-ms-interpolation-mode: nearest-neighbor;" />
    ExtL(u16),
}

impl Key {
    #[inline(always)]
    pub(crate) fn to(&self) -> u64 {
        match self {
            Key::ExtL(a) => 0b1u64 << a,
            Key::ExtR(a) => 0b1_00000000_00000000_00000000_00000000u64 << a,
            Key::Hi => Key::ExtR(26).to(),
            Key::Lo => Key::ExtR(27).to(),
            Key::Do => Key::ExtR(28).to(),
            Key::Ok => Key::ExtR(29).to(),
            Key::Exec => Key::ExtR(30).to(),
            Key::Near => Key::ExtR(31).to(),
            Key::Up => Key::ExtL(26).to(),
            Key::Down => Key::ExtL(27).to(),
            Key::Left => Key::ExtL(28).to(),
            Key::Right => Key::ExtL(29).to(),
            Key::Back => Key::ExtL(30).to(),
            Key::Far => Key::ExtL(31).to(),
        }
    }

    /// A very simple "get whether or not a key is currently being held down".
    #[inline(always)]
    pub fn held(&self, controller: usize) -> bool {
        let input = unsafe { HID_STATE.as_mut().unwrap()[controller].input };

        (input & (self.to())) != 0
    }

    // Get whether or not a key has just been modified.
    #[inline(always)]
    fn just(&self, controller: usize) -> (bool, bool) {
        let memory = unsafe { HID_STATE.as_mut().unwrap()[controller].memory };

        let mem = (memory & (self.to())) != 0;
        let get = self.held(controller);

        (get, mem ^ get)
    }

    /// Get whether a press "event" has just happenned.
    pub fn pressed(&self, controller: usize) -> bool {
        self.just(controller) == (true, true)
    }

    /// Get whether a release "event" has just happenned.
    pub fn released(&self, controller: usize) -> bool {
        self.just(controller) == (false, true)
    }
}

/// Different Outputs for HID.
#[repr(u32)]
enum Output {
    /// Vibrate Controller in the HID.
    HapticStart = 0b__0000_0000__0000_0000__0000_0000__0000_0001,
    /// Vibrate Controller in the HID.
    HapticStop = 0b__0000_0000__0000_0000__0000_0000__0000_0010,
}

/// An abstract input state which encompasses all HID's.
///
/// # Abstraction over platform differences, explained.
/// Here is the default settings for which keys are equivalent to which across platform boundaries.
/// > * Arrow Keys = D Pad
/// > * Enter,Shift,Space,Tab = ABXY
/// > * Escape Key = Back Key
/// > * Cmd = Ctrl
/// > * WASD = Move Stick
/// > * IJKL = Camera Stick
/// > * Scroll Up = L
/// > * Scroll Down = R
// 8 * 32 bits (256 bits = 32 bytes).
#[repr(C)]
#[derive(Clone)]
pub(crate) struct HidState {
    /// 32 bits memory for simulating unicode text input.
    text: char,
    /// Output.
    output: u32,

    /// Input: Binary key states.
    pub input: u64,
    /// Input: Memory
    pub memory: u64,

    /// Left C-Pad X, guaranteed Range [-1 .. 1]
    pub lstick_x: f32,
    /// Left C-Pad Y, guaranteed Range [-1 .. 1]
    pub lstick_y: f32,

    /// Right C-Pad X, guaranteed Range [-1 .. 1]
    pub rstick_x: f32,
    /// Right C-Pad Y, guaranteed Range [-1 .. 1]
    pub rstick_y: f32,
}

impl HidState {
    #[inline(always)]
    fn held(&self, key: Key) -> bool {
        let input = self.input;

        (input & (key.to())) != 0
    }

/*    // Get whether or not a key has just been modified.
    #[inline(always)]
    fn just(&self, key: Key) -> (bool, bool) {
        let memory = self.memory;

        let mem = (memory & (key.to())) != 0;
        let get = self.held(key);

        (get, mem ^ get)
    }

    /// Get whether a press "event" has just happenned.
    fn pressed(&self, key: Key) -> bool {
        self.just(key) == (true, true)
    }

    /// Get whether a release "event" has just happenned.
    fn released(&self, key: Key) -> bool {
        self.just(key) == (false, true)
    }*/
//
#[inline(always)]
fn key_set(&mut self, key: Key, pushed: bool) {
if pushed {
self.key_press(key)
} else {
self.key_release(key)
}
}

//
#[inline(always)]
fn key_toggle(&mut self, key: Key, pushed: bool) {
if pushed {
if self.held(key) {
self.key_release(key)
} else {
self.key_press(key)
}
}
}

// Set a key true.
#[allow(unused)]
#[inline(always)]
pub(crate) fn key_press(&mut self, key: Key) {
self.input |= key.to();
}

// Set a key false.
#[allow(unused)]
#[inline(always)]
pub(crate) fn key_release(&mut self, key: Key) {
self.input &= !(key.to());
}
}

// ////////////////////////////////////////////////////////////////////////////////////////// //
//                                   Public Functions                                         //
// ////////////////////////////////////////////////////////////////////////////////////////// //

/// Get the M-Stick, Left Stick X & Y (WASD, Main movement).  Ranges are [-1 .. 1].
pub fn lstick(controller: usize) -> (f32, f32) {
unsafe {
(
HID_STATE.as_mut().unwrap()[controller].lstick_x,
HID_STATE.as_mut().unwrap()[controller].lstick_y,
)
}
}

/// Get the C-Stick, Right Stick X & Y (Mouse Movement, Camera-POV movement).  Ranges are [-1 .. 1].
pub fn rstick(controller: usize) -> (f32, f32) {
unsafe {
(
HID_STATE.as_mut().unwrap()[controller].rstick_x,
HID_STATE.as_mut().unwrap()[controller].rstick_y,
)
}
}

/// Get text input (One character per frame max, \0 if nothing).
pub fn text(controller: usize) -> char {
unsafe { HID_STATE.as_mut().unwrap()[controller].text }
}

// ////////////////////////////////////////////////////////////////////////////////////////////// //
//                                   Private Functions                                            //
// ////////////////////////////////////////////////////////////////////////////////////////////// //

#[doc(hidden)]
pub(crate) mod private {
use super::*;

#[inline(always)]
pub fn new() -> NativeManager {
unsafe {
HID_STATE = Some(vec![HidState {
text: '\0',
output: 0,
input: 0,
memory: 0,
lstick_x: 0.0,
lstick_y: 0.0,
rstick_x: 0.0,
rstick_y: 0.0,
}]);
}

NativeManager::new()
}

// Reset, and get new input.
#[inline(always)]
pub fn update(c_manager: &mut NativeManager) {
let (device_count, added) = c_manager.search();

if added != ::std::usize::MAX {
unsafe {
HID_STATE.as_mut().unwrap().resize(
device_count,
HidState {
text: '\0',
output: 0,
input: 0,
memory: 0,
lstick_x: 0.0,
lstick_y: 0.0,
rstick_x: 0.0,
rstick_y: 0.0,
},
);
}
}

for i in 0..device_count {
// Copy old input into memory
unsafe {
HID_STATE.as_mut().unwrap()[i].memory = HID_STATE.as_mut().unwrap()[i].input;
}

// Check if it's unplugged
let (fd, is_out, ne) = c_manager.get_fd(i);

if ne {
continue;
}
if is_out {
c_manager.disconnect(fd);
continue;
}

unsafe {
c_manager.poll_event(i, &mut HID_STATE.as_mut().unwrap()[i]);
}
}
}
}

// Set a key true.
#[allow(unused)]
#[inline(always)]
pub(crate) fn key_press(controller: usize, key: Key) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].key_press(key);
}
}

// Set a key false.
#[allow(unused)]
#[inline(always)]
pub(crate) fn key_release(controller: usize, key: Key) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].key_release(key);
}
}

// Set left stick.  `x` & `y` are clamped [-1 .. 1].
#[allow(unused)]
#[inline(always)]
pub(crate) fn set_lstick(controller: usize, x: f32, y: f32) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].lstick_x = x.min(1.0).max(-1.0);
HID_STATE.as_mut().unwrap()[controller].lstick_y = y.min(1.0).max(-1.0);
}
}

// Set right stick.  `x` & `y` are clamped [-1 .. 1].
#[allow(unused)]
#[inline(always)]
pub(crate) fn set_rstick(controller: usize, x: f32, y: f32) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].rstick_x = x.min(1.0).max(-1.0);
HID_STATE.as_mut().unwrap()[controller].rstick_y = y.min(1.0).max(-1.0);
}
}

// Set unicode character input.
#[allow(unused)]
#[inline(always)]
pub(crate) fn set_char(controller: usize, x: char) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].text = x;
}
}

/*// Get the haptic feedback bit.
#[allow(unused)]
#[inline(always)]
pub(crate) fn get_haptic(controller: usize) -> bool {
    let output = unsafe { HID_STATE.as_mut().unwrap()[controller].output };

    (output & (Output::HapticStart as u32)) != 0
}*/
/// Start a haptic rumble effect (vibrate).
pub fn rumble_start(controller: usize) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].output |= Output::HapticStart as u32;
}
}

/// Stop a haptic rumble effect (vibrate).
pub fn rumble_stop(controller: usize) {
unsafe {
HID_STATE.as_mut().unwrap()[controller].output |= Output::HapticStop as u32;
}
}

pub(crate) trait CoordToFloat {
fn to_f32(self) -> f32;
}

impl CoordToFloat for u16 {
fn to_f32(self) -> f32 {
self as f32
}
}

impl CoordToFloat for i16 {
fn to_f32(self) -> f32 {
self as f32
}
}

#[cfg(feature = "screen")]
pub(crate) fn cursor_coordinates<T, U>(wh: (T, T), xy: (U, U))
where
U: CoordToFloat,
T: CoordToFloat,
{
let x = xy.0.to_f32();
let y = xy.1.to_f32();
let w = wh.0.to_f32();
let h = wh.1.to_f32();
let xy = (x * 2.0 / w - 1.0, y * 2.0 / h - 1.0);

if xy.0 > 1.0 || xy.0 < -1.0 || xy.1 > 1.0 || xy.1 < -1.0 {
} else {
unsafe {
HID_STATE.as_mut().unwrap()[0].screen_x = xy.0;
HID_STATE.as_mut().unwrap()[0].screen_y = xy.1;
}
}
}

#[cfg(test)]
mod tests {
/*    #[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}*/
}*/
