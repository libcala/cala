use std::collections::VecDeque;

// The program data context.
struct Data {
    buffer: VecDeque<(i16, i16)>,
}

// Set the home loop to `run()`.
cala::loop_init!(run, Data {
    buffer: VecDeque::new(),
});

fn run(data: &mut Data) -> cala::Loop<Data> {
    // Record some sound.
    cala::record(&mut |_whichmic, l, r| {
        data.buffer.push_back((l, r));
    });

    // Play that sound.
    cala::play(&mut || {
        if let Some((lsample, rsample)) = data.buffer.pop_front() {
            cala::AudioSample::stereo(lsample, rsample)
        } else {
            // Play silence if not enough has been recorded yet.
            cala::AudioSample::stereo(0, 0)
        }
    });

    cala::Continue
}
