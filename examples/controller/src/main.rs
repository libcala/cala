use cala::*;
use input::{Input, GameInput};

// Set the home loop to `run()`.
exec!(input);

async fn input() {
    while run().await { }
}

// Function that runs while your app runs.
async fn run() -> bool {
    match cala::input::input().await {
        Input::Game(_id, GameInput::Back) => return false,
        Input::Game(id, input) => println!("{}: {:?}", id, input),
        _input => {}
    }
    true
}
