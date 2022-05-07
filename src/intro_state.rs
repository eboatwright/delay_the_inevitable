use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;
use crate::GameState;
use macroquad::prelude::*;
use crate::Context;
use crate::state::*;

const INTRO_TEXT: [&'static str; 5] = [
	"Welcome.",
	"You must save",
	"our planet,",
	"By delaying a ship,",
	"Called The Inevitable.",
];

pub struct IntroState {
	pub current_text_index: usize,
}

impl IntroState {
	pub fn new() -> Self {
		Self {
			current_text_index: 0,
		}
	}
}

impl State for IntroState {
	fn init(&mut self, _: &mut Context) {
	}

	fn update(&mut self, context: &mut Context) -> UpdateStatus {
		if is_key_pressed(KeyCode::Space)
		|| is_key_pressed(KeyCode::Z) {
			self.current_text_index += 1;
			if self.current_text_index >= 5 {
				return UpdateStatus::ChangeState(Box::new(GameState::new()));
			}
			play_sound(context.resources.splash_sfx.unwrap(), PlaySoundParams {
				volume: 0.05,
				looped: false,
			});
		}

		UpdateStatus::Ok
	}

	fn render(&self, context: &Context) {
		let text = INTRO_TEXT[self.current_text_index];
		let dimensions = measure_text(text, Some(context.resources.font), 16, 1.0);
		draw_text_ex(text, 123.0 - (dimensions.width * 0.5).round(), 134.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.3, g: 0.3, b: 0.4, a: 0.5 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});
		draw_text_ex(text, 123.0 - (dimensions.width * 0.5).round(), 132.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
			color: Color { r: 0.388, g: 0.60, b: 1.0, a: 1.0 },
			font_size: 16,
			font: context.resources.font,
			..Default::default()
		});
	}
}