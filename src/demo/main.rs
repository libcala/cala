extern crate aldaron;

use aldaron::print as print;
use aldaron::screen::wayland::window as window;

fn main() {
	print::print();
	window::init();
}
