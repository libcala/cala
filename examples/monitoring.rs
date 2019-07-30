//! Audio monitoring program.  Audio from microphones sent directly to headphones.

use cala::*;

use std::collections::VecDeque;
 
fn main() {
    let mut app = App::new(());

    let mut buffer = VecDeque::new();
 
    loop {
        // Record some sound.
        app.record(&mut |_whichmic, l, r| {
            buffer.push_back((l, r));
        });

        // Play that sound.
        app.play(&mut || {
            if let Some((lsample, rsample)) = buffer.pop_front() {
                AudioSample::stereo(lsample, rsample)
            } else {
                // Play silence if not enough has been recorded yet.
                AudioSample::stereo(0, 0)
            }
        });
    }
}
