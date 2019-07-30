use crate::user::*;
use crate::audio::*;

/// The Dive Application Context.
pub struct App<T> {
    user: User,
    mic: MicrophoneSystem,
    speaker: SpeakerSystem,
    file: T,
    changed: bool,
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
        where for<'de> U: serde::Deserialize<'de>
    {
        stronghold::fetch(res)
    }

    /// Load file `res` from `zip`.
    pub fn open(&mut self, zip: &str, res: &str) -> Option<()>
        where for<'de> T: serde::Deserialize<'de>
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
        where T: serde::Serialize
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
}
