use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use crate::intro_state::IntroState;
use macroquad::prelude::*;
use crate::delta_time;
use crate::Context;
use crate::state::*;

const TIME: f32 = 600.0;

pub struct SplashState {
	pub timer: f32,
	pub played_sound: bool,
}

impl SplashState {
	pub fn new() -> Self {
		Self {
			timer: TIME,
			played_sound: false,
		}
	}
}

impl State for SplashState {
	fn init(&mut self, _: &mut Context) {
	}

	fn update(&mut self, context: &mut Context) -> UpdateStatus {
		self.timer -= delta_time();
		if self.timer <= 0.0
		|| is_key_pressed(KeyCode::Space)
		|| is_key_pressed(KeyCode::Z) {
			return UpdateStatus::ChangeState(Box::new(IntroState::new()));
		}

		if !self.played_sound
		&& self.timer <= TIME * 540.0 {
			self.played_sound = true;
			play_sound(context.resources.splash_sfx.unwrap(), PlaySoundParams {
				volume: 0.3,
				looped: false,
			});
		}

		UpdateStatus::Ok
	}

	fn render(&self, context: &Context) {
		if self.played_sound {
			draw_text_ex("eboatwright", 60.0, 134.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
				color: Color { r: 0.3, g: 0.3, b: 0.4, a: 0.5 },
				font_size: 16,
				font: context.resources.font,
				..Default::default()
			});
			draw_text_ex("eboatwright", 60.0, 132.0 + (f32::sin(get_time() as f32 * 3.0) * 10.0).round(), TextParams {
				color: Color { r: 0.388, g: 0.60, b: 1.0, a: 1.0 },
				font_size: 16,
				font: context.resources.font,
				..Default::default()
			});
		}
	}
}