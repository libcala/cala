pub(crate) use stick::Port as ControllerPort;
pub(crate) use stick::Btn;

pub(crate) enum Axis {
    JoyXY,
    CamXY,
    Lrt,
    Pitch,
    Yaw,
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

/// Select which buttons and axis you want on your controller.
///
/// Button names are from this contoller layout:
///
/// <img src="https://jeronaldaron.github.io/cala/res/controller.png" width="292">
pub struct ControllerLayout {
    pub(crate) joy: Option<bool>,
    pub(crate) cam: Option<bool>,
    pub(crate) lrt: Option<bool>,
    pub(crate) pitch: Option<bool>,
    pub(crate) yaw: Option<bool>,
    pub(crate) abxy: Option<bool>,
    pub(crate) dpad: Option<bool>,
    pub(crate) quit: Option<bool>,
    pub(crate) menu: Option<bool>,
    pub(crate) wz: Option<bool>,
    pub(crate) cd: Option<bool>,
    pub(crate) lrb: Option<bool>,
    pub(crate) axis: Vec<Axis>,
    pub(crate) btns: Vec<Btns>,
}

impl ControllerLayout {
    /// Create a new `ControllerLayout`.
    pub fn new() -> Self {
        ControllerLayout {
            joy: None,
            cam: None,
            lrt: None,
            pitch: None,
            yaw: None,
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
        if self.joy.is_some() { return self; }

        self.joy = Some(optional);
        self.axis.push(Axis::JoyXY);
        self
    }

    /// Request an x & y axis for camera (secondary) joystick.
    pub fn cam(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.cam.is_some() { return self; }

        self.cam = Some(optional);
        self.axis.push(Axis::CamXY);
        self
    }

    /// Request an x & y axis for camera (secondary) joystick.
    pub fn lrt(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.lrt.is_some() { return self; }

        self.cam = Some(optional);
        self.axis.push(Axis::Lrt);
        self
    }

    /// Request an axis for pitch (stationary throttle).
    pub fn pitch(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.pitch.is_some() { return self; }

        self.pitch = Some(optional);
        self.axis.push(Axis::Pitch);
        self
    }

    /// Request an axis for yaw (stationary throttle).
    pub fn yaw(mut self, optional: bool) -> Self {
        // Don't do twice!
        if self.yaw.is_some() { return self; }

        self.yaw = Some(optional);
        self.axis.push(Axis::Yaw);
        self
    }

    /// Request ABXY buttons.
    pub fn abxy(mut self, optional: bool) -> Self {
        if self.abxy.is_some() { return self; }

        self.abxy = Some(optional);
        self.btns.push(Btns::Abxy);
        self
    }

    /// Request arrow buttons.
    pub fn arrow(mut self, optional: bool) -> Self {
        if self.dpad.is_some() { return self; }

        self.dpad = Some(optional);
        self.btns.push(Btns::Dpad);
        self
    }

    /// Request Back button.
    pub fn back(mut self, optional: bool) -> Self {
        if self.quit.is_some() { return self; }

        self.quit = Some(optional);
        self.btns.push(Btns::Quit);
        self
    }

    /// Request Menu button.
    pub fn menu(mut self, optional: bool) -> Self {
        if self.menu.is_some() { return self; }

        self.menu = Some(optional);
        self.btns.push(Btns::Menu);
        self
    }

    /// Request W & Z buttons.
    pub fn wz(mut self, optional: bool) -> Self {
        if self.wz.is_some() { return self; }

        self.wz = Some(optional);
        self.btns.push(Btns::Wz);
        self
    }

    /// Request D & C buttons (Push in joystick).
    pub fn dc(mut self, optional: bool) -> Self {
        if self.cd.is_some() { return self; }

        self.cd = Some(optional);
        self.btns.push(Btns::Dc);
        self
    }

    /// Request L & R buttons.
    pub fn lrb(mut self, optional: bool) -> Self {
        if self.lrb.is_some() { return self; }

        self.lrb = Some(optional);
        self.btns.push(Btns::Lr);
        self
    }
}
