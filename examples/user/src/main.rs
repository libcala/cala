// Set the home loop to `run()`.
cala::loop_init!(run, ());

// Function that runs while your app runs.
pub fn run(_: &mut ()) -> cala::Loop<()> {
    // Print out the user's information.
    println!("{}", cala::user());
    // Exit.
    cala::Exit
}
