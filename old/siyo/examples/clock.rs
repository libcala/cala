extern crate siyo;

use siyo::clock::*;

fn main() {
    // Current time.
    let clock = Clock::new();
    println!("Current time (Local): {}", clock);
    println!("Current time (UTC):   {:?}", clock);

    let mut a = 0;

    println!("Print 'Hello, world #!' every {} seconds", SECOND / 3);

    loop {
        let now = Clock::new();
        let b = now.since(&clock, SECOND / 3);
        if a != b {
            a = b;
            println!("Hello, world {}!", a);
        }
    }
}
