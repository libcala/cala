//! Get joystick input.

use cala::*;

fn main() {
    let mut app = App::new(());

    let layout = cala::ControllerLayout::new().joy(false).abxy(false);

    loop {
        for id in 0..app.controller_update() {
            let state = app.controller_get(id, &layout);
            println!("{}: {:?}", id, state);
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
