// "stick" Source Code - Licensed under the MIT LICENSE (see /LICENSE)

use std::mem;
use std::ffi::CString;
use std::fs;

#[repr(C)]
struct TimeVal {
	tv_sec: isize,
	tv_usec: isize,
}

#[repr(C)]
struct Event {
	ev_time: TimeVal,
	ev_type: i16,
	ev_code: i16,
	ev_value: i32,
}

extern {
	fn open(pathname: *const u8, flags: i32) -> i32;
	fn close(fd: i32) -> i32;
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
	fn read(fd: i32, buf: *mut Event, count: usize) -> isize;
}

struct Device {
	name: Option<String>,
	fd: i32,
    min: i32,
    max: i32,
    id: i32,
    effect: i16,
}

impl PartialEq for Device {
	fn eq(&self, other: &Device) -> bool {
		if let Some(ref name) = self.name {
			if let Some(ref name2) = other.name {
				name == name2
			} else {
				false
			}
		} else {
			false
		}
	}
}

pub struct NativeManager {
	devices: Vec<Device>,
}

impl NativeManager {
	pub(crate) fn new() -> NativeManager {
		NativeManager { devices: Vec::new() }
	}

	/// Do a search for controllers.  Returns number of controllers.
	pub(crate) fn search(&mut self) -> (usize, usize) {
		let devices = find_devices();

		// Add devices
		for mut i in devices {
			if self.devices.contains(&i) {
				continue;
			}

			open_joystick(&mut i);

			// Setup device for asynchronous reads
			if i.fd != -1 {
                println!("New Joystick");

				joystick_async(i.fd);

				let index = self.add(i);
                let (min, max, _) = joystick_abs(self.devices[index].fd);
                let (_, id, _) = joystick_id(self.devices[index].fd);
                let effect = joystick_haptic(self.devices[index].fd);

                self.devices[index].min = min;
                self.devices[index].max = max;
                self.devices[index].id = id;
                self.devices[index].effect = effect;

				return (self.devices.len(), index);
			}
		}

		(self.num_plugged_in(), ::std::usize::MAX)
	}

	pub(crate) fn get_id(&self, id: usize) -> (i32, bool) {
		if id >= self.devices.len() {
			(0, true)
		} else {
			let (_, a, b) = joystick_id(self.devices[id].fd);

			(a, b)
		}
	}

	pub(crate) fn get_fd(&self, id: usize) -> (i32, bool, bool) {
		let (_, unplug) = self.get_id(id);

		(self.devices[id].fd, unplug, self.devices[id].name == None)
	}

	pub(crate) fn num_plugged_in(&self) -> usize {
		self.devices.len()
	}

	pub(crate) fn disconnect(&mut self, fd: i32) -> () {
		for i in 0..self.devices.len() {
			if self.devices[i].fd == fd {
				joystick_drop(fd);
				self.devices[i].name = None;
				return;
			}
		}

		panic!("There was no fd of {}", fd);
	}

	pub(crate) fn poll_event(&self, i: usize, state: &mut crate::hid::HidState) {
        if (state.output & crate::hid::Output::HapticStart as u32) != 0 {
            self.rumble(i, true);
            state.output = 0;
        } else if (state.output & crate::hid::Output::HapticStop as u32) != 0 {
            self.rumble(i, false);
            state.output = 0;
        }

		while joystick_poll_event(self.devices[i].fd, state, self.devices[i].min, self.devices[i].max, self.devices[i].id) {
        }
	}

	fn add(&mut self, device: Device) -> usize {
		let mut r = 0;

		for i in &mut self.devices {
			if i.name == None {
				*i = device;
				return r;
			}

			r += 1;
		}

		self.devices.push(device);

		r
	}

    pub(crate) fn rumble(&self, i: usize, on: bool) {
//        println!("RMBLE {}", on);
        joystick_rumble(self.devices[i].fd, self.devices[i].effect, on);
    }
}
impl Drop for NativeManager {
	fn drop(&mut self) -> () {
		while let Some(device) = self.devices.pop() {
			self.disconnect(device.fd);
		}
	}
}

// Find the evdev device.
fn find_devices() -> Vec<Device> {
	let mut rtn = Vec::new();
	let paths = fs::read_dir("/dev/input/by-id/");
	let paths = if let Ok(paths) = paths {
		paths
	} else {
		return vec![];
	};

	for path in paths {
		let path_str = path.unwrap().path();
		let path_str = path_str.to_str().unwrap();

		// An evdev device.
		if path_str.ends_with("-event-joystick") {
			rtn.push(Device {
				name: Some(path_str.to_string()),
				fd: -1,
                min: 0,
                max: 0,
                id: 0,
                effect: -1,
			});
		}
	}

	rtn
}

// Open the evdev device.
fn open_joystick(device: &mut Device) -> () {
	let file_name = CString::new(device.name.clone().unwrap()).unwrap();

	device.fd = unsafe {
		open(file_name.as_ptr() as *const _, 0x0002 /*Read & Write*/)
	};
}

// From: https://github.com/torvalds/linux/blob/master/include/uapi/linux/input.h

#[repr(C)]
struct FfTrigger {
    button: u16,
    interval: u16,
}

#[repr(C)]
struct FfReplay {
    length: u16,
    delay: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfEnvelope {
	attack_length: u16,
	attack_level: u16,
	fade_length: u16,
	fade_level: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfConstantEffect {
    level: i16,
    envelope: FfEnvelope,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfRampEffect {
	start_level: i16,
	end_level: i16,
	envelope: FfEnvelope,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfPeriodicEffect {
	waveform: u16,
	period: u16,
	magnitude: i16,
	offset: i16,
	phase: u16,

	envelope: FfEnvelope,

	custom_len: u32,
	custom_data: *mut i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfConditionEffect {
	right_saturation: u16,
	left_saturation: u16,

	right_coeff: i16,
	left_coeff: i16,

	deadband: u16,
	center: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfRumbleEffect {
	strong_magnitude: u16,
	weak_magnitude: u16,
}

#[repr(C)]
union FfUnion {
	constant: FfConstantEffect,
	ramp: FfRampEffect,
	periodic: FfPeriodicEffect,
	condition: [FfConditionEffect; 2], /* One for each axis */
	rumble: FfRumbleEffect,
}

#[repr(C)]
struct FfEffect {
    stype: u16,
    id: i16,
    direction: u16,

    trigger: FfTrigger,
    replay: FfReplay,

	u: FfUnion
}

fn joystick_rumble(fd: i32, id: i16, value: bool) {
    #[repr(C)]
    struct InputEvent {
        sec: usize,
        usec: usize,
	    stype: u16,
	    code: i16,
	    value: i32,
    };

    let play = InputEvent {
        sec: 0,
        usec: 0,
	    stype: 0x15 /*EV_FF*/,
	    code: id,
	    value: if value { 1 } else { 0 },
    };

    extern "C" {
        fn write(fildes: i32, buf: *const InputEvent, nbyte: usize) -> isize;
    }

    unsafe {
        write(fd, &play, std::mem::size_of::<InputEvent>());
    }
}

fn joystick_haptic(fd: i32) -> i16 {
	let mut a = FfEffect {
        stype: 0x51,
        id: -1 /*allocate new effect*/,
        direction: 0,

        trigger: FfTrigger {
            button: 0,
            interval: 0,
        },
        replay: FfReplay {
            length: 0,
            delay: 0,
        },

	    u: FfUnion {
            periodic: FfPeriodicEffect {
                waveform: 0x5a, // Sine Wave
                period: 100, /*milliseconds*/
                magnitude: 0x7FFF, /*peak value*/
                offset: 0, /*mean value of wave*/
                phase: 0, /*horizontal shift*/
                envelope: FfEnvelope {
                	attack_length: 100,
	                attack_level: 0,
	                fade_length: 100,
	                fade_level: 0,
                },
            	custom_len: 0,
	            custom_data: std::ptr::null_mut(),
            }
        }
    };

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut FfEffect) -> i32;
	}

	if unsafe { ioctl(fd, 0x40304580, &mut a) } == -1 {
        println!("Joystick Doesn't Support Rumble!");
        return -1;
	} else {
        println!("Detected Rumble Support!");
    }

    a.id
}

// Set up file descriptor for asynchronous reading.
fn joystick_async(fd: i32) -> () {
	let error = unsafe {
		fcntl(fd, 0x4, 0x800)
	} == -1;

	if error {
		panic!("Joystick unplugged 2!");
	}
}

// Get the joystick id.
fn joystick_id(fd: i32) -> (i16, i32, bool) {
	let mut a = [0i16; 4];

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut i16) -> i32;
	}

	if unsafe { ioctl(fd, 0x80084502, &mut a[0]) } == -1 {
		return (0, 0, true)
	}

	(a[0], ((a[1] as i32) << 16) | (a[2] as i32), false)
}

fn joystick_abs(fd: i32) -> (i32, i32, bool) {
	#[repr(C)]
	struct AbsInfo {
		value: i32,
		minimum: i32,
		maximum: i32,
		fuzz: i32,
		flat: i32,
		resolution: i32,
	}

	let mut a = unsafe { mem::uninitialized() };

	extern "C" {
		fn ioctl(fd: i32, request: usize, v: *mut AbsInfo) -> i32;
	}

	if unsafe { ioctl(fd, 0x80184540, &mut a) } == -1 {
		return (0, 0, true)
	}

	(a.minimum, a.maximum, false)
}

// Disconnect the joystick.
fn joystick_drop(fd: i32) -> () {
	if unsafe { close(fd) == -1 } {
		panic!("Failed to disconnect joystick.");
	}
}

// Transform joystick coordinates.
fn transform(min: i32, max: i32, val: i32, id: i32) -> f32 {
	let range = max - min;
	let value = val - min; // 0 - range
	let value = (value as f32) / (range as f32); // 0 - 1
	let value = (value * 2.0) - 1.0; // -1 to 1
	let value = (value * 100.0) as i32;

	// deadzone
	if value < 10 && value > -10 {
		0.0
	} else {
        let a = (value as f32) / match id {
            /*GameCube Controller*/ 0x791844 => 60.0,
            _ => 100.0,
        };
//        println!("{}", a);
        a
	}
}

fn joystick_poll_event(fd: i32, state: &mut crate::hid::HidState, min: i32, max: i32, id: i32) -> bool {
	let mut js = unsafe { mem::uninitialized() };

	let bytes = unsafe {
		read(fd, &mut js, mem::size_of::<Event>())
	};

	if bytes != (mem::size_of::<Event>() as isize) {
		return false;
	}

    match id {
        0x54c0268 => { include!("evdev_mappings/ps.rs") }
        _ => { /*include!("evdev_mappings/default.rs")*/ panic!("TODO") }
    }


/*	match js.ev_type {
		// button press / release (key)
		0x01 => {
			if js.ev_value == 1 {
                println!("Press {}", js.ev_code - 0x120);
			    match js.ev_code - 0x120 {
				    0 => state.key_press(crate::hid::Key::Execute),
				    1 => state.key_press(crate::hid::Key::Accept),
				    2 => state.key_press(crate::hid::Key::Control),
				    3 => state.key_press(crate::hid::Key::Action),
				    4 => state.key_press(crate::hid::Key::L),
				    5 => state.key_press(crate::hid::Key::R),
                    6 => { // ZL = Z + L
                        state.key_press(crate::hid::Key::L);
                        state.key_press(crate::hid::Key::Cmd)
                    }
				    7 => { // ZR = Z + R
                        state.key_press(crate::hid::Key::R);
                        state.key_press(crate::hid::Key::Cmd)
                    },
				    9 => state.key_press(crate::hid::Key::Back),
				    12 => state.key_press(crate::hid::Key::Up),
                    13 => state.key_press(crate::hid::Key::Down),
                    14 => state.key_press(crate::hid::Key::Left),
                    15 => state.key_press(crate::hid::Key::Right),
				    a => {} /*println!("Unknown Button: {}", a)*/,
			    }
            } else {
                println!("Release {}", js.ev_code - 0x120);
			    match js.ev_code - 0x120 {
				    0 => state.key_release(crate::hid::Key::Execute),
				    1 => state.key_release(crate::hid::Key::Accept),
				    2 => state.key_release(crate::hid::Key::Control),
				    3 => state.key_release(crate::hid::Key::Action),
				    4 => state.key_release(crate::hid::Key::L),
				    5 => state.key_release(crate::hid::Key::R),
                    6 => { // ZL = Z + L
                        state.key_release(crate::hid::Key::L);
                        state.key_release(crate::hid::Key::Cmd)
                    }
				    7 => { // ZR = Z + R
                        state.key_release(crate::hid::Key::R);
                        state.key_release(crate::hid::Key::Cmd)
                    },
				    9 => state.key_release(crate::hid::Key::Back),
				    12 => state.key_release(crate::hid::Key::Up),
                    13 => state.key_release(crate::hid::Key::Down),
                    14 => state.key_release(crate::hid::Key::Left),
                    15 => state.key_release(crate::hid::Key::Right),
				    a => {} /*println!("Unknown Button: {}", a)*/,
			    }
            }
		}
		// axis move (abs)
		0x03 => {
            println!("Axis {}", js.ev_code);

			let value = transform(min, max, js.ev_value as i32, id);

			match js.ev_code {
				0 => state.lstick_x = value.min(1.0).max(-1.0),
				1 => state.lstick_y = value.min(1.0).max(-1.0),
				2 => state.rstick_y = value.min(1.0).max(-1.0),
				3 => state.l_throttle = value.min(1.0).max(-1.0),
				4 => state.r_throttle = value.min(1.0).max(-1.0),
				5 => state.rstick_x = value.min(1.0).max(-1.0),
                16 => {
                    if js.ev_value > 0 {
                        state.key_press(crate::hid::Key::Right)
                    } else if js.ev_value < 0 {
                        state.key_press(crate::hid::Key::Left)
                    } else {
                        state.key_release(crate::hid::Key::Right);
                        state.key_release(crate::hid::Key::Left)
                    }
                },
                17 => {
                    if js.ev_value > 0 {
                        state.key_press(crate::hid::Key::Down)
                    } else if js.ev_value < 0 {
                        state.key_press(crate::hid::Key::Up)
                    } else {
                        state.key_release(crate::hid::Key::Down);
                        state.key_release(crate::hid::Key::Up)
                    }
                },
				40 => { /*ignore precision axis*/ },
				a => {} /* println!("Unknown Axis: {}", a) */,
			}
		}
		// ignore
		_ => {}
	}*/

	true
}
