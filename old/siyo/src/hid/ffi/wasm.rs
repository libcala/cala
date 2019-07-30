pub struct NativeManager {
}

impl NativeManager {
	pub(crate) fn new() -> NativeManager {
		NativeManager { /*devices: Vec::new()*/ }
	}

	/// Do a search for controllers.  Returns number of controllers.
	pub(crate) fn search(&mut self) -> (usize, usize) {
/*		let devices = find_devices();

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
		}*/

		(self.num_plugged_in(), ::std::usize::MAX)
	}

	pub(crate) fn get_id(&self, id: usize) -> (i32, bool) {
//		if id >= self.devices.len() {
			(0, true)
/*		} else {
			let (_, a, b) = joystick_id(self.devices[id].fd);

			(a, b)
		}*/
	}

	pub(crate) fn get_fd(&self, id: usize) -> (i32, bool, bool) {
/*		let (_, unplug) = self.get_id(id);

		(self.devices[id].fd, unplug, self.devices[id].name == None)*/
        (0, false, false)
	}

	pub(crate) fn num_plugged_in(&self) -> usize {
//		self.devices.len()
        0
	}

	pub(crate) fn disconnect(&mut self, _fd: i32) -> () {
/*		for i in 0..self.devices.len() {
			if self.devices[i].fd == fd {
				joystick_drop(fd);
				self.devices[i].name = None;
				return;
			}
		}

		panic!("There was no fd of {}", fd);*/
	}

	pub(crate) fn poll_event(&self, _i: usize, _state: &mut crate::hid::HidState) {
/*        if (state.output & crate::hid::Output::HapticStart as u32) != 0 {
            self.rumble(i, true);
            state.output = 0;
        } else if (state.output & crate::hid::Output::HapticStop as u32) != 0 {
            self.rumble(i, false);
            state.output = 0;
        }

		while joystick_poll_event(self.devices[i].fd, state, self.devices[i].min, self.devices[i].max, self.devices[i].id) {
        }*/
	}

/*	fn add(&mut self, _device: Device) -> usize {
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
        0
	}*/

    pub(crate) fn rumble(&self, _i: usize, _on: bool) {
//        println!("RMBLE {}", on);
//        joystick_rumble(self.devices[i].fd, self.devices[i].effect, on);
    }
}
