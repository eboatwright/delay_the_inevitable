use macroquad::prelude::*;
use crate::UpdateStatus;
use crate::Context;
use crate::State;

pub struct WinState {
}

impl WinState {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl State for WinState {
	fn init(&mut self, _: &mut Context) {
	}

	fn update(&mut self, _: &mut Context) -> UpdateStatus {
		UpdateStatus::Ok
	}

	fn render(&self, context: &Context) {
		draw_text_ex("Thanks for playing!", 20.0, 124.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.3, g: 0.3, b: 0.4, a: 0.5 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});

		draw_text_ex("Made for LD 50", 34.0, 140.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.3, g: 0.3, b: 0.4, a: 0.5 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});

		draw_text_ex("Thanks for playing!", 20.0, 122.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.388, g: 0.60, b: 1.0, a: 1.0 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});

		draw_text_ex("Made for LD 50", 34.0, 138.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.388, g: 0.60, b: 1.0, a: 1.0 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});
	}
}