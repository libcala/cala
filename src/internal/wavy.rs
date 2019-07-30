// This is one of the global contexts of Cala.  This one is particularly fast because it
// is read only, and multiple threads can read the data at the same time.

use wavy::MicrophoneSystem;
use wavy::SpeakerSystem;

pub use wavy::SampleRate;
pub use wavy::AudioSample;

struct AudioIO {
    speaker: std::sync::Mutex<SpeakerSystem>,
    mic: std::sync::Mutex<MicrophoneSystem>,
}

static mut AUD_IO: FakeAudioIO = FakeAudioIO([0; std::mem::size_of::<AudioIO>()]);

#[repr(align(8))]
struct FakeAudioIO([u8; std::mem::size_of::<AudioIO>()]);

// // // // // //

pub(crate) fn initialize_audio_io() {
    use wavy::*;

    unsafe {
        let aud_io = &mut AUD_IO as *mut _ as *mut AudioIO;

        std::ptr::write(aud_io, AudioIO {
            speaker: std::sync::Mutex::new(SpeakerSystem::new(SampleRate::Normal).unwrap()),
            mic: std::sync::Mutex::new(MicrophoneSystem::new(SampleRate::Normal).unwrap()),
        });
    }
}

/// Set the `SampleRate` for playing.
pub fn set_play_hz(sr: SampleRate) {
    let aud_io = unsafe { &mut AUD_IO as *mut _ as *mut AudioIO };
    unsafe { *(*aud_io).speaker.lock().unwrap() = SpeakerSystem::new(sr).unwrap(); }
}

/// Set the `SampleRate` for recording.
pub fn set_record_hz(sr: SampleRate) {
    let aud_io = unsafe { &mut AUD_IO as *mut _ as *mut AudioIO };
    unsafe { *(*aud_io).mic.lock().unwrap() = MicrophoneSystem::new(sr).unwrap(); }
}

/// Play Audio.  Callback generates audio samples sent directly to speakers.
pub fn play(callback: &mut FnMut() -> AudioSample) {
    let aud_io = unsafe { &mut AUD_IO as *mut _ as *mut AudioIO };
    unsafe { (*aud_io).speaker.lock().unwrap().play(callback) };
}

/// Record Audio.  Callback's parameters are (microphone ID, left sample, right sample).
pub fn record(callback: &mut FnMut(usize, i16, i16)) {
    let aud_io = unsafe { &mut AUD_IO as *mut _ as *mut AudioIO };
    unsafe { (*aud_io).mic.lock().unwrap().record(callback) };
}
