//! Print out user information.

use cala::*;

fn main() {
    let app = App::new(());

    println!("{}", app.user());
}
