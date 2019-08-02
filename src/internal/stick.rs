// This is one of the global contexts of Cala.

pub(crate) use stick::Btn;
pub(crate) use stick::Port as ControllerPort;
pub(crate) use stick::CONTROLLER_MAX as MAX;

/// The maximum number of controllers allowed.
pub const CONTROLLER_MAX: u8 = MAX as u8;

pub(crate) enum Axis {
    JoyXY,
    CamXY,
    Lrt,
}

pub(crate) enum Btns {
    Abxy,
    Dpad,
    Quit,
    Menu,
    Wz,
    Lr,
    Dc,
}

/// Iterator over controllers.  Use [`controllers()`](fn.controllers.html) to get.
pub struct ControllerIter<'a> {
    id: u8,
    i: u8,
    layout: &'a ControllerLayout,
}

/// Get an iterator over controller IDs.  **Warning**: They are not guarenteed to be
/// consecutive, even though they usually are.
pub fn controllers(layout: &'_ ControllerLayout) -> ControllerIter<'_> {
    ControllerIter {
        id: 0, i: 0,
        layout,
    }
}

impl<'a> Iterator for ControllerIter<'a> {
    type Item = (u8, ControllerState);

    fn next(&mut self) -> Option<(u8, ControllerState)> {
        // Search for controller.
        loop {
            // We need to check if the controller has been unplugged.
            if self.id == CONTROLLER_MAX || self.i == controller_count() {
                return None;
            }

            // Get controller at i, if it exists, otherwise keep looking.
            if let Some(state) = controller_get(self.id, &self.layout) {
                self.i += 1;
                return Some((self.id, state));
            }
            self.id += 1;
        }
    }
}

/// State of a controller.
pub struct ControllerState {
    axis_count: usize,
    axis: [f32; 8],
    btns_count: usize,
    btns: [bool; 16],
}

impl ControllerState {
    /// Return the controller state as a tuple of axis and buttons.
    pub fn get(&self) -> (&[f32], &[bool]) {
        (&self.axis[..self.axis_count], &self.btns[..self.btns_count])
    }
}

/// Select which buttons and axis you want on your controller.
///
/// Button names are from this contoller layout:
///
/// <img src="https://jeronaldaron.github.io/cala/res/controller.png" width="292">
pub struct ControllerLayout {
    joy: Option<bool>,
    cam: Option<bool>,
    lrt: Option<bool>,
    abxy: Option<bool>,
    dpad: Option<bool>,
    quit: Option<bool>,
    menu: Option<bool>,
    wz: Option<bool>,
    cd: Option<bool>,
    lrb: Option<bool>,
    axis: Vec<Axis>,
    btns: Vec<Btns>,
}

impl Default for ControllerLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerLayout {
    /// Create a new `ControllerLayout`.
    pub fn new() -> Self {
        ControllerLayout {
            joy: None,
            cam: None,
            lrt: None,
            abxy: None,
            dpad: None,
            quit: None,
            menu: None,
            wz: None,
            cd: None,
            lrb: None,
            axis: vec![],
            btns: vec![],
        }
    }

    /// Request an x & y axis for main joystick.
    pub fn joy(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.joy.is_some() {
            return self;
        }

        self.joy = Some(optional);
        self.axis.push(Axis::JoyXY);
        self
    }

    /// Request an x & y axis for camera (secondary) joystick.
    pub fn cam(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.cam.is_some() {
            return self;
        }

        self.cam = Some(optional);
        self.axis.push(Axis::CamXY);
        self
    }

    /// Request an x & y axis for camera (secondary) joystick.
    pub fn lrt(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.lrt.is_some() {
            return self;
        }

        self.cam = Some(optional);
        self.axis.push(Axis::Lrt);
        self
    }

    /// Request ABXY buttons.
    pub fn abxy(mut self, optional: bool) -> Self {
        if self.abxy.is_some() {
            return self;
        }

        self.abxy = Some(optional);
        self.btns.push(Btns::Abxy);
        self
    }

    /// Request arrow buttons.
    pub fn arrow(mut self, optional: bool) -> Self {
        if self.dpad.is_some() {
            return self;
        }

        self.dpad = Some(optional);
        self.btns.push(Btns::Dpad);
        self
    }

    /// Request Back button.
    pub fn back(mut self, optional: bool) -> Self {
        if self.quit.is_some() {
            return self;
        }

        self.quit = Some(optional);
        self.btns.push(Btns::Quit);
        self
    }

    /// Request Menu button.
    pub fn menu(mut self, optional: bool) -> Self {
        if self.menu.is_some() {
            return self;
        }

        self.menu = Some(optional);
        self.btns.push(Btns::Menu);
        self
    }

    /// Request W & Z buttons.
    pub fn wz(mut self, optional: bool) -> Self {
        if self.wz.is_some() {
            return self;
        }

        self.wz = Some(optional);
        self.btns.push(Btns::Wz);
        self
    }

    /// Request D & C buttons (Push in joystick).
    pub fn dc(mut self, optional: bool) -> Self {
        if self.cd.is_some() {
            return self;
        }

        self.cd = Some(optional);
        self.btns.push(Btns::Dc);
        self
    }

    /// Request L & R buttons.
    pub fn lrb(mut self, optional: bool) -> Self {
        if self.lrb.is_some() {
            return self;
        }

        self.lrb = Some(optional);
        self.btns.push(Btns::Lr);
        self
    }
}

static mut PORT: FakePort = FakePort([0; std::mem::size_of::<ControllerPort>()]);

// Make aligned to usize.
#[repr(align(8))]
struct FakePort([u8; std::mem::size_of::<ControllerPort>()]);

// // // // // //

pub(crate) fn initialize_controller_io() {
    unsafe {
        let port = &mut PORT as *mut _ as *mut ControllerPort;

        *port = ControllerPort::new();
    }

    // Start a new thread for getting controller input.
    std::thread::spawn(controller_thread);
}

// The controller thread.
fn controller_thread() {
    let port = unsafe { &mut PORT as *mut _ as *mut ControllerPort };

    loop {
        unsafe { (*port).poll(); }
    }
}

/// Return the number of controllers.
pub fn controller_count() -> u8 {
    let port = unsafe { &PORT as *const _ as *const ControllerPort };

    // TODO: Write?
    unsafe { (*port).count() }
}

/// Get the state of a controller from the requested controller layout.
///
/// This is useful for when you want different controllers to have different layouts.  If
/// you want all of your controllers to have the same layout, then you should get a
/// [`ControllerIter`](struct.ControllerIter.html) from
/// [`controllers()`](fn.controllers.html).
///
/// # Usage
/// ```rust
/// // Set the home loop to `run()`.
/// cala::init!(run, ());
/// 
/// // Function that runs while your app runs.
/// pub fn run(_: &mut ()) -> cala::Loop<()> {
///     let layout = cala::ControllerLayout::new().joy(false).lrt(false).abxy(false);
///     let mut i = 0;
/// 
///     // Iterate through all of the controllers.
///     'a: for id in 0..cala::controller_count() {
///         // Search for controller.
///         'b: loop {
///             // Get controller at i, if it exists, otherwise keep looking.
///             if let Some(state) = cala::controller_get(i, &layout) {
///                 // Found the controller with id = `id`.
///                 println!("{}→{}: {:?}", id, i, state.get());
///                 break 'b;
///             }
///             i += 1;
///             // We need to check if the controller has been unplugged.
///             if i == cala::CONTROLLER_MAX {
///                 break 'a;
///             }
///         }
///     }
///     std::thread::sleep(std::time::Duration::from_millis(16));
///     // Exit.
///     cala::Continue
/// }
/// ```
pub fn controller_get(id: u8, layout: &ControllerLayout) -> Option<ControllerState> {
    let port = unsafe { &PORT as *const _ as *const ControllerPort };

    let mut axis: [f32; 8] = [0.0; 8];
    let mut btns: [bool; 16] = [false; 16];

    let state = unsafe { (*port).get(id) }?;
    let mut i_axis = 0;
    let mut i_btns = 0;

    for axis_number in 0..layout.axis.len() {
        match layout.axis[axis_number] {
            Axis::JoyXY => {
                // TODO: Fallback.
                let (x, y) = state.joy().unwrap_or((0.0, 0.0));
                axis[i_axis] = x;
                i_axis += 1;
                axis[i_axis] = y;
                i_axis += 1;
            }
            Axis::CamXY => {
                // TODO: Fallback.
                let (x, y) = state.cam().unwrap_or((0.0, 0.0));
                axis[i_axis] = x;
                i_axis += 1;
                axis[i_axis] = y;
                i_axis += 1;
            }
            Axis::Lrt => {
                // TODO: Fallback.
                let (x, y) = state.lrt().unwrap_or((0.0, 0.0));
                axis[i_axis] = x;
                i_axis += 1;
                axis[i_axis] = y;
                i_axis += 1;
            }
        }
    }

    for btn in 0..layout.btns.len() {
        match layout.btns[btn] {
            Btns::Abxy => {
                btns[i_btns] = state.btn(Btn::A).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::B).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::X).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::Y).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Dpad => {
                btns[i_btns] = state.btn(Btn::Up).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::Down).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::Left).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::Right).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Quit => {
                btns[i_btns] = state.btn(Btn::E).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Menu => {
                btns[i_btns] = state.btn(Btn::F).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Wz => {
                btns[i_btns] = state.btn(Btn::W).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::Z).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Lr => {
                btns[i_btns] = state.btn(Btn::L).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::R).unwrap_or(false);
                i_btns += 1;
            }
            Btns::Dc => {
                btns[i_btns] = state.btn(Btn::D).unwrap_or(false);
                i_btns += 1;
                btns[i_btns] = state.btn(Btn::C).unwrap_or(false);
                i_btns += 1;
            }
        }
    }

    Some(ControllerState {
        axis,
        btns,
        axis_count: i_axis,
        btns_count: i_btns,
    })
}
