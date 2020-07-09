use cala::*;

const INFO: &str = "Info";

fn main() {
    journal::out!(INFO, "User message");
    journal::dev!(INFO, "Developer message");
}
