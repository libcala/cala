use crate::audio::*;
use crate::controller::*;
use crate::user::*;

/// The Dive Application Context.
pub struct App<T> {
    // User
    user: User,
    // Audio
    mic: MicrophoneSystem,
    speaker: SpeakerSystem,
    // Store
    file: T,
    changed: bool,
    // Controller
    controller_port: ControllerPort,
    axis: [f32; 8],
    btns: [bool; 16],
}

impl<T> App<T> {
    /// Create a new Application.
    pub fn new(file: T) -> App<T> {
        App {
            user: User::new(),
            mic: MicrophoneSystem::new(SampleRate::Normal).unwrap(),
            speaker: SpeakerSystem::new(SampleRate::Normal).unwrap(),
            file,
            changed: false,
            controller_port: ControllerPort::new(),
            axis: [0.0; 8],
            btns: [false; 16],
        }
    }

    /// Get user information.
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Get the file data.
    pub fn file(&mut self) -> &mut T {
        &mut self.file
    }

    /// Fetch a resource.
    pub fn fetch<U>(&mut self, res: &str) -> Option<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        stronghold::fetch(res)
    }

    /// Load file `res` from `zip`.
    pub fn open(&mut self, zip: &str, res: &str) -> Option<()>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.file = stronghold::load(zip, res)?;
        Some(())
    }

    /// File data has changed.  Next call to `sync()` will save the file.
    pub fn edit(&mut self) {
        self.changed = true;
    }

    /// Save file `res` in `zip` only if `edit()` has been called since last change.
    pub fn sync(&mut self, zip: &str, res: &str)
    where
        T: serde::Serialize,
    {
        if self.changed {
            stronghold::save(zip, res, &self.file);
            self.changed = false;
        }
    }

    /// Record Audio.  Callback's parameters are (microphone ID, left sample, right sample).
    pub fn record(&mut self, callback: &mut FnMut(usize, i16, i16)) {
        self.mic.record(callback);
    }

    /// Play Audio.  Callback generates audio samples sent directly to speakers.
    pub fn play(&mut self, callback: &mut FnMut() -> AudioSample) {
        self.speaker.play(callback);
    }

    /// Return the number of controllers.
    pub fn controller_update(&mut self) -> u16 {
        self.controller_port.update()
    }

    /// Get the state of a controller from the requested controller layout.
    pub fn controller_get(&mut self, id: u16, layout: &ControllerLayout) -> (&[f32], &[bool]) {
        let mut state = self.controller_port.get(id);
        let mut i_axis = 0;
        let mut i_btns = 0;

        for axis in 0..layout.axis.len() {
            match layout.axis[axis] {
                Axis::JoyXY => {
                    // TODO: Fallback.
                    let (x, y) = state.joy().unwrap_or((0.0, 0.0));
                    self.axis[i_axis] = x;
                    i_axis += 1;
                    self.axis[i_axis] = y;
                    i_axis += 1;
                }
                Axis::CamXY => {
                    // TODO: Fallback.
                    let (x, y) = state.cam().unwrap_or((0.0, 0.0));
                    self.axis[i_axis] = x;
                    i_axis += 1;
                    self.axis[i_axis] = y;
                    i_axis += 1;
                }
                Axis::Lrt => {
                    // TODO: Fallback.
                    let (x, y) = state.lrt().unwrap_or((0.0, 0.0));
                    self.axis[i_axis] = x;
                    i_axis += 1;
                    self.axis[i_axis] = y;
                    i_axis += 1;
                }
                Axis::Pitch => {
                    // TODO: Fallback.
                    let x = state.pitch().unwrap_or(0.0);
                    self.axis[i_axis] = x;
                    i_axis += 1;
                }
                Axis::Yaw => {
                    //                    let x = state.yaw().unwrap_or(0.0);
                    // TODO: Not supported yet.
                    self.axis[i_axis] = 0.0;
                    i_axis += 1;
                }
            }
        }

        for btn in 0..layout.btns.len() {
            match layout.btns[btn] {
                Btns::Abxy => {
                    self.btns[i_btns] = state.btn(Btn::A).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::B).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::X).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::Y).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Dpad => {
                    self.btns[i_btns] = state.btn(Btn::Up).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::Down).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::Left).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::Right).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Quit => {
                    self.btns[i_btns] = state.btn(Btn::E).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Menu => {
                    self.btns[i_btns] = state.btn(Btn::F).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Wz => {
                    self.btns[i_btns] = state.btn(Btn::W).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::Z).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Lr => {
                    self.btns[i_btns] = state.btn(Btn::L).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::R).unwrap_or(false);
                    i_btns += 1;
                }
                Btns::Dc => {
                    self.btns[i_btns] = state.btn(Btn::D).unwrap_or(false);
                    i_btns += 1;
                    self.btns[i_btns] = state.btn(Btn::C).unwrap_or(false);
                    i_btns += 1;
                }
            }
        }

        (&self.axis[..i_axis], &self.btns[..i_btns])
    }
}
