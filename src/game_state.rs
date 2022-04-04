use macroquad::audio::*;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use macroquad::prelude::*;
use crate::circles::Circle;
use crate::circles::*;
use crate::particle::*;
use crate::the_inevitable::*;
use crate::projectile::*;
use crate::stars::*;
use crate::player::*;
use crate::Context;
use crate::state::*;

pub struct GameState {
	pub stars: Vec<Star>,
	pub player: Player,
	pub the_inevitable: TheInevitable,
	pub projectiles: Vec<Projectile>,
	pub particles: Vec<Particle>,
	pub circles: Vec<Circle>,
	pub screen_shake: Vec2,
	pub is_other_frame: bool,
}

impl GameState {
	pub fn new() -> Self {
		Self {
			stars: vec![],
			player: Player::new(),
			the_inevitable: TheInevitable::new(),
			projectiles: vec![],
			particles: vec![],
			circles: vec![],
			screen_shake: Vec2::ZERO,
			is_other_frame: false,
		}
	}
}

impl State for GameState {
	fn init(&mut self, context: &mut Context) {
		stop_sound(context.resources.music.unwrap());
		play_sound(context.resources.music.unwrap(), PlaySoundParams {
			volume: 0.3,
			looped: true,
		});
		stars_init(self);
	}

	fn update(&mut self, context: &mut Context) -> UpdateStatus {
		self.is_other_frame = !self.is_other_frame;
		if self.is_other_frame {
			self.screen_shake *= -0.7;
			context.viewport.camera.target = (vec2(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5) + self.screen_shake).round();
		}

		stars_update(self);

		match player_update(self, context) {
			UpdateStatus::Ok => {}
			UpdateStatus::ChangeState(state) =>
				return UpdateStatus::ChangeState(state),
		}

		match the_inevitable_update(self, context) {
			UpdateStatus::Ok => {}
			UpdateStatus::ChangeState(state) =>
				return UpdateStatus::ChangeState(state),
		}

		projectiles_update(self, context);
		particles_update(self);
		circles_update(self);

		UpdateStatus::Ok
	}

	fn render(&self, context: &Context) {
		stars_render(self, context);
		player_render(self, context);
		projectiles_render(self, context);
		the_inevitable_render(self, context);
		particles_render(self);
		circles_render(self);

		draw_rectangle(
			-10.0, -10.0,
			SCREEN_WIDTH + 20.0, SCREEN_HEIGHT + 20.0,
			Color {
				r: 0.0,
				g: 0.0,
				b: 0.0,
				a: 1.0 - (self.the_inevitable.win_delay / 80.0),
			},
		);
	}
}