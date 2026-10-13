// SPDX-License-Identifier: GPL-2.0
/*
 * lcd1602.rs
 * 
 * Copyright (C) 2026 James Osterhage
 */

//! Driver for a 1602 LCD connected via I2C

use kernel::prelude::*;

module! {
	type: Lcd1602,
	name: "lcd_1602",
	authors: ["James Osterhage"],
	description: "Driver for a 1602 LCD connected via I2C",
	license: "GPL",
}

#[pin_data(PinnedDrop)]
struct Lcd1602 {
	#[pin]
	_miscdev: kernel::miscdevice::MiscDeviceRegistration<Lcd1602Device>,
}

impl kernel::InPlaceModule for Lcd1602 {
	fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
		pr_info!("LCD1602 loaded\n");

		let options = kernel::miscdevice::MiscDeviceOptions {
			name: kernel::c_str!("lcd"),
		};

		try_pin_init!(Self {
			_miscdev <- kernel::miscdevice::MiscDeviceRegistration::register(options),
		})
	}
}

#[pinned_drop]
impl PinnedDrop for Lcd1602 {
	fn drop(self: Pin<&mut Self>) {
		pr_info!("LCD1602 driver unloaded\n");
	}
}

struct Lcd1602Device {}

#[vtable]
impl kernel::miscdevice::MiscDevice for Lcd1602Device {
	type Ptr = Pin<KBox<Self>>;

	fn open(
		_file: &kernel::fs::File,
		_misc: &kernel::miscdevice::MiscDeviceRegistration<Self>,
	) -> Result<Pin<KBox<Self>>> {
		return Err(ENOTTY);
	}
}