include!(concat!(env!("OUT_DIR"), "/siyo_app.rs"));

use siyo::hid::*;
use siyo::screen::*;

pub struct App {
    r#loop: fn(app: &mut App) -> bool,
}

fn new() -> App {
    App {
        r#loop: main_loop,
    }
}
 
fn run(app: &mut App) -> bool {
    (app.r#loop)(app)
}
 
fn main_loop(app: &mut App) -> bool {
    /*// Check for exit request
    if Key::Back.held(0) {
        println!("Back pressed, so quiting....");
        return false;
    }*/

    true
}
