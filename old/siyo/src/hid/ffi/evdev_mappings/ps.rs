if js.ev_type == 0x01 { // button press / release (key)
    let key = js.ev_code - 0x120;
    let push = js.ev_value == 1;
    match key {
        17 => state.key_set(crate::hid::Key::Ok, push),
        16 => state.key_set(crate::hid::Key::Lo, push),
        20 => state.key_set(crate::hid::Key::Do, push),
        19 => state.key_set(crate::hid::Key::Hi, push),
        22 => state.key_set(crate::hid::Key::Back, push), // L1
        23 => state.key_set(crate::hid::Key::Exec, push), // R1
        24 => state.key_set(crate::hid::Key::Far, push), // L2
        25 => state.key_set(crate::hid::Key::Near, push), // R2
        26 => state.key_set(crate::hid::Key::Exec, push), // R1- Select
        27 => state.key_set(crate::hid::Key::Back, push), // Start 
        28 => state.key_set(crate::hid::Key::Back, push), // P3
        29 => state.key_toggle(crate::hid::Key::Near, push), //  LStickPress
        30 => state.key_toggle(crate::hid::Key::Lo, push), //  RStickPress
        256 => state.key_set(crate::hid::Key::Up, push),
        257 => state.key_set(crate::hid::Key::Down, push),
        258 => state.key_set(crate::hid::Key::Left, push),
        259 => state.key_set(crate::hid::Key::Right, push),
        _ => {}
    }
} else if js.ev_type == 0x03 { // axis move
	let value = transform(min, max, js.ev_value as i32, id);

    match js.ev_code {
		0 => state.lstick_x = value.min(1.0).max(-1.0),
		1 => state.lstick_y = value.min(1.0).max(-1.0),
		3 => state.rstick_x = value.min(1.0).max(-1.0),
		4 => state.rstick_y = value.min(1.0).max(-1.0),
		_ => {}
    }
}
