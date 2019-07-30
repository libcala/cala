// This example program may be used for any purpose proprietary or not.

#[macro_use]
extern crate siyo;

use siyo::hid;

main!(
    Ctx::new,
    struct Ctx {
        // The mode
        mode: fn(app: &mut Ctx) -> bool,
    }
);

impl Ctx {
    fn new() -> Ctx {
        Ctx {
            mode: mode,
        }
    }

    fn run(&mut self) -> bool {
        (self.mode)(self)
    }
}

fn test(key: hid::Key) {
    if key.pressed(0) {
        println!("{:?} Press", key);
    }

    if key.released(0) {
        println!("{:?} Release", key);
    }
}

// Code that runs every frame.
fn mode(_app: &mut Ctx) -> bool {
    // Check for exit request
    if hid::Key::Back.held(0) {
        println!("Back pressed, so quiting....");
        return false;
    }

    test(hid::Key::Exec);
    if hid::Key::Ok.pressed(0) {
        hid::rumble_start(0);
        println!("Accept Press");
    }

    if hid::Key::Ok.released(0) {
        hid::rumble_stop(0);
        println!("Accept Release");
    }
    test(hid::Key::Hi);
    test(hid::Key::Lo);
    test(hid::Key::Do);

    test(hid::Key::Up);
    test(hid::Key::Down);
    test(hid::Key::Left);
    test(hid::Key::Right);

    test(hid::Key::Near);
    test(hid::Key::Far);

    //    println!("{} {}", hid::lstick(0).0, hid::lstick(0).1);

    true
}
