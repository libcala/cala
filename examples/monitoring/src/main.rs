use std::cell::RefCell;
use cala::*;
use microphone::{S16LEx2, Recorder};
use speaker::Player;

/// Shared data between recorder and player.
struct Shared {
    /// A stereo audio buffer.
    buffer: Vec<S16LEx2>,
}
 
exec!(monitor);
async fn monitor() {
    /// Extend buffer by slice of new frames from last plugged in device.
    async fn record(shared: &RefCell<Shared>) {
        let mut recorder = Recorder::<S16LEx2>::new().unwrap();
        loop {
            let _sample_rate = recorder.fut().await;
            let shared: &mut Shared = &mut *shared.borrow_mut();
            recorder.record_last(&mut shared.buffer);
        }
    }
    /// Drain double ended queue frames into last plugged in device.
    async fn play(shared: &RefCell<Shared>) {
        let mut player = Player::<S16LEx2>::new().unwrap();
        loop {
            let _sample_rate = player.fut().await;
            let shared: &mut Shared = &mut *shared.borrow_mut();
            let n_frames = player.play_last(shared.buffer.as_slice());
            shared.buffer.drain(..n_frames.min(shared.buffer.len()));
        }
    }
 
    let shared = RefCell::new(Shared { buffer: Vec::new() });
    let mut record = record(&shared);
    let mut play = play(&shared);
    println!("Entering async loop…");
    [record.fut(), play.fut()].select().await;
    unreachable!()
}
