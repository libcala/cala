// Set the home loop to `run()`.
cala::loop_init!(run, ());

// Function that runs while your app runs.
pub fn run(_: &mut ()) -> cala::Loop<()> {
    let layout = cala::ControllerLayout::new().joy(false).lrt(false).abxy(false);

    // Iterate through all of the controllers.
    'a: for (id, state) in cala::controllers(&layout) {
        println!("{}: {:?}", id, state.get());
    }
    std::thread::sleep(std::time::Duration::from_millis(16));
    // Exit.
    cala::Continue
}
