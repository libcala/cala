use cala::*;

// Set the home loop to `run()`.
exec!(run);

// Function that runs while your app runs.
pub async fn run() {
    // Print out the user's information.
    println!("{}", user::realname());
}
