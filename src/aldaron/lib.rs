extern crate libc;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
	#[cfg(mac_framework)]
	#[link(kind="framework", name="wayland-client")]
	extern {}

	#[cfg(not(mac_framework))]
	#[link(name="wayland-client")]
	extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
	#[link(name="wayland-client")]
	extern {}
}

pub mod screen;

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}*/

pub mod print {
	pub fn print() {
		println!("Hey World!");
	}
}
