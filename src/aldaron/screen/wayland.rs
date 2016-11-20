use libc::{c_int, c_char, c_void};

pub enum wl_display { }
pub enum wl_event_queue { }

#[link(name = "wayland-client")]
extern {
	pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
	pub fn wl_display_disconnect(display: *mut wl_display) -> ();
}

pub mod window {
	pub fn init() {
		// Call wayland libary's init.
//		let mut wl_display = unsafe {
//			wl_display_connect(0);
//		};
	}

	pub fn kill() {
		
	}
}
