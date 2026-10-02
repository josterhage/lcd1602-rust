//! Driver for a 1602 LCD connected via I2C

use kernel::prelude::*;

module! {
	type: Lcd1602,
	name: "lcd_1602",
	authors: ["James Osterhage"],
	description: "Driver for a 1602 LCD connected via I2C",
	license: "GPL",
}

struct Lcd1602;

impl kernel::Module for Lcd1602 {
	fn init(_module: &'static ThisModule) -> Result<Self> {
		pr_info!("Lcd1602 driver loaded\n");
		Ok(Lcd1602)
	}
}

impl Drop for Lcd1602 {
	fn drop(&mut self) {
		pr_info!("Lcd1602 driver unloaded\n");
	}
}